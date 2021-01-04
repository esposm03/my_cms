use actix_web::HttpResponse;

/// A really simple route, that always returns 200, just for health checking
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
