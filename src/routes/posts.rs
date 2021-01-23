//! Routes related to posts

use actix_web::{
    web::{Data, Json, Path},
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

/// Create a new post and return its id
///
/// This route should be mounted as `POST /post`
#[tracing::instrument(name = "Adding a new post", fields(title = %post.title), skip(conn, post))]
pub async fn create_post(post: Json<PostSubmitData>, conn: Data<PgPool>) -> impl Responder {
    match insert_post(&post, &conn).await {
        Ok(uuid) => Ok(uuid.to_string()),
        Err(_) => Err(HttpResponse::InternalServerError()),
    }
}

/// Retrieve a post from its id, saved in the query as `post_id`
///
/// This route should be mounted as a `GET /post`
#[tracing::instrument(name = "Requesting a post", skip(conn))]
pub async fn get_post(id: Path<Uuid>, conn: Data<PgPool>) -> impl Responder {
    let query = sqlx::query_as!(
        PostReturnData,
        r#"SELECT id, title, content, created FROM posts WHERE id = $1"#,
        *id,
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

/// Retrieve all the posts
///
/// This route should be mounted as `GET /posts`
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

/// Data returned to the user when fetching a post
#[derive(Deserialize, Serialize, Debug)]
pub struct PostReturnData {
    pub title: String,
    pub content: String,
    pub created: DateTime<Utc>,
    pub id: Uuid,
}

/// Data submitted by a user when creating a post
#[derive(Deserialize, Serialize, Debug)]
pub struct PostSubmitData {
    pub title: String,
    pub content: String,
}
