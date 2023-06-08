//! tests/health_check.rs

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
// Launch our application in the background
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // Retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let api = server::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(api);
    // We return the application address to the caller!
    format!("http://127.0.0.1:{}", port)
}
