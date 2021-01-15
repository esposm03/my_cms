use chrono::Utc;
use uuid::Uuid;
use std::collections::HashMap;
use juniper::{EmptySubscription, FieldResult, GraphQLInputObject, GraphQLObject, Variables, graphql_object, graphql_value};

use crate::storage::Storage;

/// A post
#[derive(Clone, Debug, Default, GraphQLObject)]
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

impl From<NewPost> for Post {
    fn from(input: NewPost) -> Self {
        Post {
            id: Uuid::new_v4(),
            created: Utc::now().to_rfc3339(),
            title: input.title,
            content: input.content,
            tags: input.tags,
        }
    }
}

/// A post
#[derive(Clone, Debug, Default, GraphQLInputObject)]
pub struct NewPost {
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

#[graphql_object(context = Storage)]
impl Tag {
    fn name(&self) -> &str {
        &self.name
    }

    fn posts(&self, storage: &Storage) -> Vec<Post> {
        self.posts
            .iter()
            .filter_map(|id| storage.posts().get(id).cloned())
            .collect()
    }
}

struct Query;

#[graphql_object(context = Storage)]
impl Query {
    fn post(storage: &Storage, id: Uuid) -> Option<Post> {
        storage.posts().get(&id).cloned()
    }

    fn posts(storage: &Storage) -> Vec<Post> {
        storage.posts().values().cloned().collect()
    }
}

struct Mutation;

#[graphql_object(context = Storage)]
impl Mutation {
    fn new_post(storage: &Storage, post: NewPost) -> FieldResult<Post> {
        let converted: Post = post.into();
        storage.posts_mut().insert(converted.id, converted.clone());
        Ok(converted)
    }
}

type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Storage>>;

#[test]
pub fn run_graphql() {
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

    let storage = Storage::new(tags, posts);
    let variables = Variables::new();

    let (result, _) = juniper::execute_sync(
        r#"
        mutation AddPost {
            newPost(post: {
                title: "lorem"
                content: "ipsum"
                tags: []
            }) {
                id
                title
                content
            }
        }
        "#,
        None,
        &Schema::new(Query, Mutation, Default::default()),
        &variables,
        &storage,
    ).unwrap();

    let (result, _) = juniper::execute_sync(
        r"{ posts { id, title } }",
        None,
        &Schema::new(Query, Mutation, Default::default()),
        &Variables::new(),
        &storage,
    ).unwrap();

    println!("{}", result);
}