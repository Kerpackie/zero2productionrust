// Import necessary components from the Actix Web framework
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

// Import the Server type — this is what gets returned by our `run` function
use actix_web::dev::Server;

// Import the standard (synchronous) TcpListener from the Rust standard library
use std::net::TcpListener;

//// HANDLER FUNCTION //// 

// This is an asynchronous handler function for the `/health_check` route
// It takes an HttpRequest (even though we don’t use it here), and returns a 200 OK response
async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish() // Responds with an empty body and 200 OK status
}

//// APPLICATION STARTUP FUNCTION //// 

// This function sets up and runs the HTTP server
// It takes a TcpListener, which specifies the address and port to listen on
// It returns a Result that either contains the Server or an I/O error
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    // Create the Actix Web server using a closure that builds the application for each worker thread
    let server = HttpServer::new(|| {
        // Build the application with one route: /health_check
        App::new()
            // Register the route handler for GET requests to /health_check
            .route("/health_check", web::get().to(health_check))
    })
        // Tell the server to listen on the provided TCP listener
        // The listener may be bound to a specific port (e.g., 127.0.0.1:8000) or a random one (127.0.0.1:0)
        .listen(listener)?
        // Start the server — this returns a Server handle that we can await
        .run();

    // Return the server so the caller can await or manage it
    Ok(server)
}
