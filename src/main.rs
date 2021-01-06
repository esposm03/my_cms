use my_cms::{
    configuration::get_configuration,
    run,
    telemetry::{get_subscriber, init_subscriber},
};
use sqlx::PgPool;
use std::net::TcpListener;

#[actix_rt::main]
async fn main() {
    // Setup logging with `tracing`
    let subscriber = get_subscriber("my_cms".into(), "info".into());
    init_subscriber(subscriber);

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
