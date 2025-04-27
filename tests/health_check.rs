use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client.get(format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to send GET /health_check request");
    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Error binding to port");
    let port = listener.local_addr().unwrap().port();

    let server = zero2prod::run(listener).expect("Failed to bind server");
    std::mem::drop(tokio::spawn(server));
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn post_subscribe_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let body = "name=Khang%20Nguyen&email=khang%40gmail.com";

    let response = client
        .post(format!("{}/subscriptions", address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to send POST /subscriptions");
    assert!(response.status().is_success());
}

#[tokio::test]
async fn post_subscribe_fails() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = [
        ("name=Khang", "missing email"),
        ("email=khang%40gmail.com", "missing name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("{}/subscriptions", address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to send POST request");

        assert_eq!(
            response.status().as_u16(),
            400,
            "The API did not fail when the body is {}", error_message
        )

    }
}
