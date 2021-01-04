use my_cms::{
    configuration::{get_configuration, DatabaseSettings},
    run,
};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;

#[actix_rt::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", app.address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[actix_rt::test]
async fn create_post_valid() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .post(&format!("{}/post", app.address))
        .body(r#"{"title": "lorem ipsum", "content": "dolor sit amet"}"#)
        .header("Content-Type", "application/json")
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);

    let saved = sqlx::query!("SELECT title, content FROM posts")
        .fetch_one(&app.db_pool)
        .await
        .unwrap();

    assert_eq!(saved.title, "lorem ipsum");
    assert_eq!(saved.content, "dolor sit amet");
}

#[actix_rt::test]
async fn create_post_missing_fields() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let test_cases = [
        (r#"{"content": "dolor sit amet"}"#, "missing title"),
        (r#"{"title": "lorem ipsum"}"#, "missing content"),
        (r#"{}"#, "missing all data"),
    ];

    for (body, error_message) in &test_cases {
        let response = client
            .post(&format!("{}/post", app.address))
            .body(*body)
            .header("Content-Type", "application/json")
            .send()
            .await
            .unwrap();

        assert_eq!(
            response.status(),
            400,
            "Request succeded while {}",
            error_message
        );
    }
}

/// Run an instance of our API, without blocking the current thread,
/// and return its address. Each instance will have its own logical database
async fn spawn_app() -> TestApp {
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
