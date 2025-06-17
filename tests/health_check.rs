use std::net::TcpListener;

// This attribute tells Tokio to run the test function asynchronously
#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app(); // Only need to call this once

    // Create an HTTP client using the `reqwest` crate
    let client = reqwest::Client::new();

    // Act: Send a GET request to /health_check on our test server
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert: Check that the response is 200 OK and has no content
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// This function spawns the application server in the background
// and returns the address it is listening on (e.g., http://127.0.0.1:PORT)
fn spawn_app() -> String {
    // Bind to a random available port on localhost
    let tcp_listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address");

    // Retrieve the assigned port number
    let port = tcp_listener.local_addr().unwrap().port();

    // Start the server using your application’s run() function
    let server = zero2productionrust::run(tcp_listener).expect("Failed to start server");

    // Run the server in the background using Tokio
    tokio::spawn(server);

    // Return the full address to use in HTTP requests
    format!("http://127.0.0.1:{}", port)
}
