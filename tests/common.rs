use my_cms::{
    configuration::{get_configuration, DatabaseSettings},
    run,
    telemetry::{get_subscriber, init_subscriber},
};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;

lazy_static::lazy_static! {
    static ref TRACING: () = {
        let filter = if std::env::var("TEST_LOG").is_ok() { "debug" } else { "" };
        let subscriber = get_subscriber("test".into(), filter.into());
        init_subscriber(subscriber);
    };
}

/// Run an instance of our API, without blocking the current thread,
/// and return its address. Each instance will have its own logical database
pub async fn spawn_app() -> TestApp {
    lazy_static::initialize(&TRACING);

    let listener = TcpListener::bind("127.0.0.1:0").expect("Can't bind to random port");
    let port = listener.local_addr().unwrap().port();
    let mut config = get_configuration().unwrap();

    let db_pool = configure_database(&mut config.database).await;
    let clone = db_pool.clone();

    // Spawn an instance of our app without blocking
    actix_rt::spawn(async move { run(listener, clone).unwrap().await.unwrap() });

    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        db_pool,
    }
}

/// For test isolation, we create a logical database for each one, and return
/// a pool of connections to it. Each database will have a random Uuid v4 as name
///
/// Note that this will modify the config that's passed, so it can be used afterwards
async fn configure_database(config: &mut DatabaseSettings) -> PgPool {
    config.database_name = Uuid::new_v4().to_string();

    // Create the logical db
    let query: &str = &format!("CREATE DATABASE \"{}\"", config.database_name);
    PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres")
        .execute(query)
        .await
        .expect("Failed to create temporary database");

    // Run migrations on the database
    let pool = PgPool::connect(&config.connection_string()).await.unwrap();
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations on database");
    pool
}

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}
