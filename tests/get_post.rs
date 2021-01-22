mod common;
use common::{insert_post, spawn_app};

use my_cms::routes::PostReturnData;
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

    let response: PostReturnData = response.json().await.unwrap();
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
