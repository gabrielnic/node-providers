//! TODO
//  - [ ] add a method to return all NP at once? Maybe there's a better way

use axum::{routing::get, Router};
use tower_http::cors::{Any, CorsLayer};

async fn get_node_providers() {}

#[tokio::main]
async fn main() {
    // Create a CORS layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build our application with routes
    let app = Router::new()
        .route("/api/node-providers", get(get_node_providers))
        .layer(cors);

    // Start the server
    let addr = "127.0.0.1:8080";
    println!("Starting server at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
