use actix_web::{
    web::{Data, Json, Query},
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

/// Create a new post in the database, with the given data as input.
///
/// This is a handler: it is called by `actix` when a matching request arrives,
/// and as such returns an `HttpResponse`.
#[tracing::instrument(
    name = "Adding a new post",
    skip(post, conn),
    fields(request_id = %Uuid::new_v4(), title = %post.title)
)]
pub async fn create_post(post: Json<PostData>, conn: Data<PgPool>) -> Resp {
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
            let conv = PostData {
                title: r.title,
                content: r.content,
                date: Some(r.created.to_rfc3339()),
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
async fn insert_post(post: &PostData, connection: &PgPool) -> Result<Uuid, sqlx::Error> {
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
pub struct PostData {
    pub title: String,
    pub content: String,
    pub date: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PostId {
    id: Uuid,
}
