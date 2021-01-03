use my_cms::{configuration::get_configuration, run};
use sqlx::PgPool;
use std::net::TcpListener;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", configuration.app_port);

    let listener = TcpListener::bind(address)?;
    let connection = PgPool::connect(&configuration.database.connection_string()).await.unwrap();

    run(listener, connection)?.await
}
