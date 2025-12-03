mod db;
mod handlers;
mod models;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    // Initialize the database pool
    let pool = match db::init_pool().await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Failed to initialize database: {}", e);
            std::process::exit(1);
        }
    };

    // Set up CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build the router
    let app = Router::new()
        .route("/", get(handlers::index))
        .route("/tasks", get(handlers::list_tasks))
        .route("/tasks", post(handlers::create_task))
        .route("/tasks/{id}", put(handlers::update_task))
        .route("/tasks/{id}", delete(handlers::delete_task))
        .route("/tasks/{id}/complete", post(handlers::complete_task))
        .route("/tasks/{id}/toggle", post(handlers::toggle_task))
        .layer(cors)
        .with_state(pool);

    // Start the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("🚀 Server running at http://localhost:8080");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
