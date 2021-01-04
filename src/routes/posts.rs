use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
use std::ops::Deref;

pub async fn create_post(post: web::Json<PostData>, connection: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
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
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().finish())
}

#[derive(serde::Deserialize, Debug)]
pub struct PostData {
    title: String,
    content: String,
}
