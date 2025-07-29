use std::{net::TcpListener, sync::Arc};

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Fialed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to start server");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{port}")
}

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let response = Arc::new(
        client
            .get(format!("{address}/health_check").as_str())
            .send()
            .await
            .expect("Failed to send request"),
    );

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_vaild_from_data() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=usrula_le_guin%40gmail.com";
    let response = client
        .post(format!("{}/subscriptions", &app_address).as_str())
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to excute request");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invaild_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invaild_body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}",
            error_message
        )
    }
}
