pub mod configuration;
pub mod routes;

use actix_cors::Cors;
use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;
use std::{io, net::TcpListener};
use tracing_actix_web::TracingLogger;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, io::Error> {
    let db_pool = web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger)
            .wrap(Cors::permissive())
            .route("/health_check", web::get().to(routes::health_check))
            .route("/post", web::get().to(routes::get_post))
            .route("/post", web::post().to(routes::create_post))
            .route("/posts", web::get().to(routes::get_all_posts))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}