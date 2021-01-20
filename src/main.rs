use tracing_subscriber::fmt::Subscriber;
use sqlx::PgPool;

use my_cms::{configuration::get_configuration, run};
use std::net::TcpListener;

#[actix_rt::main]
async fn main() {
    Subscriber::builder()
        .pretty()
        .init();

    let configuration = get_configuration().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", configuration.app_port);
    let listener = TcpListener::bind(&address).expect("Failed to bind to address");
    let connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .unwrap();

    run(listener, connection)
        .expect("Failed to start server")
        .await
        .expect("Server crashed");
}
