use my_cms::{configuration::get_configuration, run};
use sqlx::{Connection, PgConnection};
use std::net::TcpListener;

#[actix_rt::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[actix_rt::test]
async fn create_post_valid() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let config = get_configuration().expect("Failed to read configuration");
    let conn_string = config.database.connection_string();
    let mut db_connection = PgConnection::connect(&conn_string).await.unwrap();

    let response = client
        .post(&format!("{}/post", app_address))
        .body(r#"{"title": "lorem ipsum", "content": "dolor sit amet"}"#)
        .header("Content-Type", "application/json")
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);

    let saved = sqlx::query!("SELECT title, content FROM posts")
        .fetch_one(&mut db_connection)
        .await
        .unwrap();

    assert_eq!(saved.title, "lorem ipsum");
    assert_eq!(saved.content, "dolor sit amet");
}

#[actix_rt::test]
async fn create_post_missing_fields() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = [
        (r#"{"content": "dolor sit amet"}"#, "missing title"),
        (r#"{"title": "lorem ipsum"}"#, "missing content"),
        (r#"{}"#, "missing all data"),
    ];

    for (body, error_message) in &test_cases {
        let response = client
            .post(&format!("{}/post", address))
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
/// and return its address
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Can't bind to random port");
    let port = listener.local_addr().unwrap().port();

    actix_rt::spawn(async { run(listener).unwrap().await.unwrap() });

    format!("http://127.0.0.1:{}", port)
}
