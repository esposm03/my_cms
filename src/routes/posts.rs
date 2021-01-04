use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use std::ops::Deref;
use uuid::Uuid;

pub async fn create_post(
    post: web::Json<PostData>,
    connection: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
    let req_span = tracing::info_span!(
        "Adding a new post",
        request_id = %Uuid::new_v4(),
        title = %post.title,
    );
    let _req_span_guard = req_span.enter();

    let query_span = tracing::info_span!("Saving a new post in the database");
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
    .execute(connection.get_ref().deref())
    .instrument(query_span)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    tracing::info!("Post '{}' has been saved", post.title);

    Ok(HttpResponse::Ok().finish())
}

#[derive(serde::Deserialize, Debug)]
pub struct PostData {
    title: String,
    content: String,
}
