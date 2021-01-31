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

use actix_cors::Cors;
use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, Responder};
use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema, SimpleObject, Value, http::playground_source};
use async_graphql_actix_web::{Request, Response};
use std::{collections::BTreeMap, io, net::TcpListener, sync::{Arc, RwLock}};
use tracing_actix_web::TracingLogger;
use uuid::Uuid;

/// Spawn the app
///
/// This function takes a [`TcpListener`] and returns a server to be
/// `.await`ed by the caller, for testing reasons.
pub fn run(listener: TcpListener) -> Result<Server, io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger)
            .wrap(Cors::permissive())
            .data(Schema::new(Query::new(), EmptyMutation, EmptySubscription))
            .route("/health_check", web::get().to(health_check))
            .route("/graphql", web::to(graphql_route))
            .route("/playground", web::get().to(index_playground))
    })
    .listen(listener)?
    .run();

    Ok(server)
}

type MySchema = Schema<Query, EmptyMutation, EmptySubscription>;

struct Structure {
    fields: Vec<String>,
    entries: BTreeMap<Uuid, Entry>,
}

impl Structure {
    fn entry_from(&mut self, fields: &String) -> Entry {
        todo!("Deserialize JSON, validate that all fields are there, and insert into the db");
    }
}

#[derive(Default, SimpleObject, Clone)]
struct Entry {
    id: Uuid,
    title: String,
    collection: String,
}

struct Query(Arc<RwLock<BTreeMap<String, Structure>>>);
impl Query {
    fn new() -> Self {
        todo!()
    }
}

#[Object]
impl Query {
    async fn entries(&self, collection: String) -> Option<Vec<Entry>> {
        self.0
            .read()
            .unwrap()
            .get(collection.as_str())
            .map(|s| s.entries.values().cloned().collect())
    }
}

struct Mutation(Arc<RwLock<BTreeMap<String, Structure>>>);
impl Mutation {
    fn new() -> Self {
        todo!()
    }
}

#[Object]
impl Mutation {
    async fn add_entry(&self, structure: String, fields: String) -> Option<Entry> {
        self.0
            .write()
            .unwrap()
            .get_mut(&structure)
            .map(|s| {
                s.entry_from(&fields)
            })
            
    }
}

async fn health_check() -> web::HttpResponse {
    web::HttpResponse::Ok().finish()
}

async fn graphql_route(schema: web::Data<MySchema>, req: Request) -> impl Responder {
    Response(schema.execute(req.into_inner()).await.into())
}

async fn index_playground() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            async_graphql::http::GraphQLPlaygroundConfig::new("/graphql"),
        ))
}
