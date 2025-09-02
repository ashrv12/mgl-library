use axum::{Json, Router, routing::get};
use colored::*;
use serde::Serialize;

#[tokio::main]
async fn main() {
    // Address of local server
    let addr = "0.0.0.0:3012";
    println!("{}", "Starting the new axum based HTTP server...".yellow());

    let app = Router::new().route("/healthz", get(healthz));

    println!("-----------------------------------");
    println!("{}", "Successfully started server".green());
    println!("The server is running on [{}]", addr.on_green());
    println!("-----------------------------------");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: String,
}

async fn healthz() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: String::from("healthy"),
    })
}

// Example of routing
// let app = Router::new()
//     .route("/", get(root))
//     .route("/foo", get(get_foo).post(post_foo))
//     .route("/foo/bar", get(foo_bar));
