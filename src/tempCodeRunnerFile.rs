use actix_web::{get, web, App, HttpServer, Responder};

// Define a simple handler for a GET request to '/'
#[get("/")]
async fn index() -> impl Responder {
    "Hello, this is your API!"
}

// Define a handler for a specific path
#[get("/greet/{name}")]
async fn greet(web::Path(name): web::Path<String>) -> impl Responder {
    format!("Hello, {}!", name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Start HTTP server and bind it to a specific IP and port
    HttpServer::new(|| {
        App::new()
            // Register the handlers
            .service(index)
            .service(greet)
    })
    .bind("127.0.0.1:8080")? // Change the IP and port as needed
    .run()
    .await
}
