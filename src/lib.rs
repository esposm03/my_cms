//! A small but fast CMS
//!
//! Warning: experimental code up ahead!
//! This project was created by me to learn some backend web development,
//! but is still only a hobby project. I tried to make it as production-ready
//! as possible, but I don't recommend you actually use it :)
//!
//! The code is pretty simple: in the `configuration` module there is the logic for using
//! configuration options from the environment (right now, only a file in the current directory)
//! by using the [`config`] crate. The module `routes`, instead, contains the bulk of the
//! application, with all the request handling and the database interactions happening there.
//! To start the server, just call the function [`run`] with a [`TcpListener`] and a database connection.

pub mod configuration;
pub mod routes;

use actix_cors::Cors;
use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;
use std::{io, net::TcpListener};
use tracing_actix_web::TracingLogger;

/// Spawn the app
///
/// This function takes a [`TcpListener`] and returns a server to be
/// `.await`ed by the caller, for testing reasons.
pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, io::Error> {
    let db_pool = web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger)
            .wrap(Cors::permissive())
            .route("/health_check", web::get().to(routes::health_check))
            .route("/post/{id}", web::get().to(routes::get_post))
            .route("/post", web::post().to(routes::create_post))
            .route("/posts", web::get().to(routes::get_all_posts))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
