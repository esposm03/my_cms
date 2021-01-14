mod common;
use common::spawn_app;

use chrono::Utc;
use my_cms::routes::Post;
use sqlx::PgPool;
use uuid::Uuid;

#[actix_rt::test]
async fn get_post_works() {
    let app = spawn_app().await;
    let id = insert_post(&app.db_pool).await;

    let response = reqwest::Client::new()
        .get(&format!("{}/post?id={}", app.address, id))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 200);

    let response: Post = response.json().await.unwrap();
    assert_eq!(response.title, "Lorem Ipsum");
    assert_eq!(response.content, "Dolor sit amet");
}

#[actix_rt::test]
async fn get_post_wrong_id() {
    let app = spawn_app().await;
    insert_post(&app.db_pool).await;
    let id = Uuid::new_v4();

    let response = reqwest::Client::new()
        .get(&format!("{}/post?id={}", app.address, id))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 404);
}

#[actix_rt::test]
async fn get_post_no_id() {
    let app = spawn_app().await;
    insert_post(&app.db_pool).await;

    let response = reqwest::Client::new()
        .get(&format!("{}/post", app.address))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 400);
}

/// Helper function. This directly inserts a post in the given
/// database, with title "Lorem Ipsum" and content "Dolor sit
/// amet", and returns its randomly-generated UUID
async fn insert_post(pool: &PgPool) -> Uuid {
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
