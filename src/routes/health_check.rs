//! Health check request

use actix_web::{HttpResponse, Responder};

/// A really simple route, that always returns 200, used to check the service status
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
