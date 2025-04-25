use reqwest;

#[tokio::test]
async fn health_check_works() {
    spawn_app();
    let client = reqwest::Client::new();
    let response = client.get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to GET request");
    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind server");
    std::mem::drop(tokio::spawn(server));
}
