use std::{
    collections::HashMap,
    sync::{Arc, Weak},
};

use actix_web::{
    web::{Data, Json, /*Query*/},
    HttpResponse,
};
use sqlx::PgPool;

use serde::{Deserialize, Serialize};
use tracing::error;
use uuid::Uuid;

// Rustc complains about when, for some reason, `query!` isn't expanded
#[allow(unused_imports)]
use chrono::Utc;

type Resp = Result<HttpResponse, HttpResponse>;

use juniper::{EmptyMutation, EmptySubscription, GraphQLObject, Variables, graphql_object};

/// A post
#[derive(Debug, Default, GraphQLObject)]
pub struct Post {
    /// The post's id
    pub id: Uuid,
    /// The creation date of the post
    pub created: String,
    /// The post's title
    pub title: String,
    /// The post's content (can include html)
    pub content: String,
    /// A list of the post's tags
    pub tags: Vec<String>,
}

/// A tag
#[derive(Debug, Default)]
pub struct Tag {
    /// The tag's displayed name
    pub name: String,
    /// The list of the tag's posts
    pub posts: Vec<Uuid>,
}

pub struct Storage {
    tags: HashMap<String, Tag>,
    posts: HashMap<Uuid, Post>,
}
impl juniper::Context for Storage {}

#[graphql_object(context = Storage)]
impl Tag {
    fn name(&self) -> &str {
        &self.name
    }

    fn posts(&self, storage: &Storage) -> Vec<&Post> {
        self.posts
            .iter()
            .filter_map(|id| storage.posts.get(id))
            .collect()
    }
}

struct Query;

#[graphql_object(context = Storage)]
impl Query {
    fn post(storage: &Storage, id: Uuid) -> Option<&Post> {
        storage.posts.get(&id)
    }

    fn posts(storage: &Storage) -> Vec<&Post> {
        storage.posts.values().collect()
    }
}

type Schema = juniper::RootNode<'static, Query, EmptyMutation<Storage>, EmptySubscription<Storage>>;

#[test]
fn run_graphql() {
    let id1 = Uuid::new_v4();
    let id2 = Uuid::new_v4();
    let id3 = Uuid::new_v4();
    let id4 = Uuid::new_v4();

    let n1 = "Hell1".into();
    let n2 = "Hell2".into();
    let n3 = "Hell3".into();
    let n4 = "Hell4".into();

    let mut posts = HashMap::new();
    posts.insert(id1, Post { id: id1, ..Default::default() });
    posts.insert(id2, Post { id: id2, ..Default::default() });
    posts.insert(id3, Post { id: id3, ..Default::default() });
    posts.insert(id4, Post { id: id4, ..Default::default() });

    let mut tags = HashMap::new();
    tags.insert(n1, Tag { posts: vec![id1], ..Default::default() });
    tags.insert(n2, Tag { posts: vec![id1, id2], ..Default::default() });
    tags.insert(n3, Tag { posts: vec![id2, id4], ..Default::default() });
    tags.insert(n4, Tag { posts: vec![id3], ..Default::default() });

    let storage = Storage { tags, posts };

    let (result, _) = juniper::execute_sync(
        r"{ posts { id, title } }",
        None,
        &Schema::new(Query, Default::default(), Default::default()),
        &Variables::new(),
        &storage,
    ).unwrap();

    println!("{}", result);
}

/*
/// Create a new post in the database, with the given data as input.
///
/// This is a handler: it is called by `actix` when a matching request arrives,
/// and as such returns an `HttpResponse`.
#[tracing::instrument(
    name = "Adding a new post",
    skip(post, conn),
    fields(request_id = %Uuid::new_v4(), title = %post.title)
)]
pub async fn create_post(post: Json<Post>, conn: Data<PgPool>) -> Resp {
    match insert_post(&post, &conn).await {
        Ok(uuid) => Ok(HttpResponse::Ok().body(uuid.to_string())),
        Err(_) => Err(HttpResponse::InternalServerError().finish()),
    }
}

/// Retrieve a post from its ID
#[tracing::instrument(name = "Requesting a post")]
pub async fn get_post(post_id: Query<PostId>, conn: Data<PgPool>) -> Resp {
    let query = sqlx::query!(
        r#"SELECT title, content, created FROM posts WHERE id = $1"#,
        post_id.id,
    );

    match query.fetch_one(&**conn).await {
        Err(sqlx::Error::RowNotFound) => Err(HttpResponse::NotFound().finish()),
        Err(e) => {
            error!("Database query failed: {:?}", e);
            Err(HttpResponse::InternalServerError().finish())
        }
        Ok(r) => {
            let conv = Post {
                title: r.title,
                content: r.content,
                created: Some(r.created.to_rfc3339()),
            };
            Ok(HttpResponse::Ok().json(conv))
        }
    }
}

/// Create a post in the database.
///
/// This is a helper function for `create_post`, which does the actual
/// insertion into the db. This is done to use the `tracing::instrument` attribute.
#[tracing::instrument(
    name = "Saving a new post in the database",
    skip(post, connection),
    fields(title = %post.title)
)]
async fn insert_post(post: &Post, connection: &PgPool) -> Result<Uuid, sqlx::Error> {
    let post_id = Uuid::new_v4();

    let res = sqlx::query!(
        r#"
            INSERT INTO posts (id, title, content, created)
            VALUES ($1, $2, $3, $4)
        "#,
        post_id,
        post.title,
        post.content,
        Utc::now(),
    )
    .execute(connection)
    .await;

    match res {
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            Err(e)
        }
        Ok(_) => Ok(post_id),
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PostId {
    id: Uuid,
}
*/