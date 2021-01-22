use actix_web::{
    web::{Data, Json, Query},
    HttpResponse, Responder,
};
use sqlx::PgPool;

use chrono::DateTime;
use serde::{Deserialize, Serialize};
use tracing::error;
use uuid::Uuid;

// Rustc complains about this when, for some reason, `query!` isn't expanded
#[allow(unused_imports)]
use chrono::Utc;

/// Create a new post in the database, with the given data as input, and return its UUID.
///
/// This is a handler: it is called by `actix` when a matching request arrives,
/// and as such returns an `HttpResponse`.
#[tracing::instrument(
    name = "Adding a new post",
    fields(request_id = %Uuid::new_v4(), title = %post.title)
    skip(conn, post),
)]
pub async fn create_post(post: Json<PostSubmitData>, conn: Data<PgPool>) -> impl Responder {
    match insert_post(&post, &conn).await {
        Ok(uuid) => Ok(uuid.to_string()),
        Err(_) => Err(HttpResponse::InternalServerError()),
    }
}

/// Retrieve a post from its ID, saved in the query as `post_id`
#[tracing::instrument(name = "Requesting a post", skip(conn))]
pub async fn get_post(post_id: Query<PostId>, conn: Data<PgPool>) -> impl Responder {
    let query = sqlx::query_as!(
        PostReturnData,
        r#"SELECT id, title, content, created FROM posts WHERE id = $1"#,
        post_id.id,
    );

    match query.fetch_one(&**conn).await {
        Err(sqlx::Error::RowNotFound) => Err(HttpResponse::NotFound()),
        Err(e) => {
            error!("Database query failed: {:?}", e);
            Err(HttpResponse::InternalServerError())
        }
        Ok(r) => Ok(Json(r)),
    }
}

/// Retrieve all of the posts in this blog
#[tracing::instrument(name = "Requesting all posts", skip(conn))]
pub async fn get_all_posts(conn: Data<PgPool>) -> impl Responder {
    let query = sqlx::query_as!(PostReturnData, "SELECT * FROM posts")
        .fetch_all(&**conn)
        .await;

    match query {
        Err(e) => {
            error!("Database query failed: {:?}", e);
            Err(HttpResponse::InternalServerError())
        }
        Ok(r) => Ok(Json(r)),
    }
}

/// Create a post in the database.
///
/// This is a helper function for `create_post`, which does the actual
/// insertion into the db. This is done to use the `tracing::instrument` attribute.
#[tracing::instrument(
    name = "Saving a new post in the database",
    skip(connection, post),
    fields(title = %post.title)
)]
async fn insert_post(post: &PostSubmitData, connection: &PgPool) -> Result<Uuid, sqlx::Error> {
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

/// The data associated with a post. `date` should contain a date
/// in ISO 8601 format, obtainable by calling the `to_rfc3339()` method
/// on any of `chrono`'s types.
#[derive(Deserialize, Serialize, Debug)]
pub struct PostReturnData {
    pub title: String,
    pub content: String,
    pub created: DateTime<Utc>,
    pub id: Uuid,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PostSubmitData {
    pub title: String,
    pub content: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PostId {
    id: Uuid,
}
