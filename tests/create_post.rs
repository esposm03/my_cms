mod common;
use common::spawn_app;

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
