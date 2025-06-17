// Import the standard library's TCP listener for binding to an IP and port
use std::net::TcpListener;

// Import the run function from your application crate
use zero2productionrust::run;

// Use Tokio's main macro to make the main function asynchronous
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Bind to IP address 127.0.0.1 on port 8000
    // This will fail (and return an error) if the port is already in use
    let listener = TcpListener::bind("127.0.0.1:8000")?;

    // Start the HTTP server using the `run` function you defined earlier
    // `run` returns a Server which is an async task, so we `.await` it to keep the server running
    run(listener)?.await
}
