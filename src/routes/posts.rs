use actix_web::{web, HttpResponse};

pub async fn create_post(_data: web::Json<PostData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize, Debug)]
pub struct PostData {
    title: String,
    content: String,
}
