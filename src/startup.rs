use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgConnection;
use std::{io, net::TcpListener, sync::Arc};

use crate::routes;

pub fn run(listener: TcpListener, db_conn: Arc<PgConnection>) -> Result<Server, io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(routes::health_check))
            .route("/post", web::post().to(routes::create_post))
            .data(db_conn.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}