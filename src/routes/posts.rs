use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

type Resp = Result<HttpResponse, HttpResponse>;

/// Create a new post in the database, with the given data as input.
///
/// This is a handler: it is called by `actix` when a matching request arrives,
/// and as such returns an `HttpResponse`.
#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(post, conn),
    fields(request_id = %Uuid::new_v4(), title = %post.title)
)]
pub async fn create_post(post: Json<PostData>, conn: Data<PgPool>) -> Resp {
    match insert_subscriber(&post, &conn).await {
        Ok(()) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Err(HttpResponse::InternalServerError().finish()),
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
async fn insert_subscriber(post: &PostData, connection: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
            INSERT INTO posts (id, title, content, created)
            VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        post.title,
        post.content,
        Utc::now(),
    )
    .execute(connection)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}

#[derive(serde::Deserialize, Debug)]
pub struct PostData {
    title: String,
    content: String,
}
