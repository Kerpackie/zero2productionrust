use std::fmt::format;
use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    spawn_app();
    // We need to bring in reqwest
    // to perform HTTP Requests against our application
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

// Launch application in background -- somehow.
fn spawn_app() -> String {
    let tcp_listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address");
    
    // Retrieve port assigned by the OS
    let port = tcp_listener.local_addr().unwrap().port();

    let server = zero2productionrust::run(tcp_listener).expect("failed to bind address");
    
    let _ = tokio::spawn(server);
    
    format!("http://127.0.0.1:{}", port)
}