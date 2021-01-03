use actix_web::{dev::Server, web, App, HttpServer};
use std::{io, net::TcpListener};

use crate::routes;

pub fn run(listener: TcpListener) -> Result<Server, io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(routes::health_check))
            .route("/post", web::post().to(routes::create_post))
    })
    .listen(listener)?
    .run();

    Ok(server)
}