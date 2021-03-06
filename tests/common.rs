use my_cms::{
    configuration::{get_configuration, DatabaseSettings},
    run,
};
use std::net::TcpListener;

use chrono::Utc;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use tracing_subscriber::{fmt::Subscriber, EnvFilter};
use uuid::Uuid;

lazy_static::lazy_static! {
    static ref TRACING: () = {
        Subscriber::builder()
            .with_env_filter(EnvFilter::from_default_env())
            .pretty()
            .init();
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

/// Helper function. This directly inserts a post in the given
/// database, with title "Lorem Ipsum" and content "Dolor sit
/// amet", and returns its randomly-generated UUID
#[allow(dead_code)]
pub async fn insert_post(pool: &PgPool) -> Uuid {
    let id = Uuid::new_v4();
    let title = "Lorem Ipsum";
    let content = "Dolor sit amet";

    let query = sqlx::query!(
        r#"
            INSERT INTO posts (id, title, content, created)
            VALUES ($1, $2, $3, $4)
        "#,
        id,
        title,
        content,
        Utc::now(),
    );

    query.execute(pool).await.unwrap();

    id
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
