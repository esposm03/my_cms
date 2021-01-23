mod common;
use common::{insert_post, spawn_app};
use my_cms::routes::PostReturnData;
use uuid::Uuid;

#[actix_rt::test]
async fn list_posts_works() {
    let app = spawn_app().await;

    let id0 = insert_post(&app.db_pool).await;
    let id1 = insert_post(&app.db_pool).await;
    let id2 = insert_post(&app.db_pool).await;

    let response = reqwest::Client::new()
        .get(&format!("{}/posts", app.address))
        .send()
        .await
        .unwrap();
    assert!(response.status().is_success());

    let response: Vec<PostReturnData> = response.json().await.unwrap();
    assert_eq!(response.len(), 3);
    assert!(post_eq(&response[0], id0));
    assert!(post_eq(&response[1], id1));
    assert!(post_eq(&response[2], id2));
}

#[actix_rt::test]
async fn list_posts_empty() {
    let app = spawn_app().await;

    let response = reqwest::Client::new()
        .get(&format!("{}/posts", app.address))
        .send()
        .await
        .unwrap();

    assert!(response.status().is_success());

    let response: Vec<PostReturnData> = response.json().await.unwrap();
    assert_eq!(response.len(), 0);
}

fn post_eq(post: &PostReturnData, id: Uuid) -> bool {
    post.id == id && post.title == "Lorem Ipsum" && post.content == "Dolor sit amet"
}
