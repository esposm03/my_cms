use my_cms::run;
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

/// Run an instance of our API, without blocking the current thread,
/// and return its address
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Can't bind to random port");
    let port = listener.local_addr().unwrap().port();

    actix_rt::spawn(async { run(listener).unwrap().await.unwrap() });

    format!("http://127.0.0.1:{}", port)
}
