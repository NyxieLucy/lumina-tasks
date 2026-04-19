mod lib;
use tower_http::cors::CorsLayer;
use tower::ServiceBuilder;
use axum::{
    routing::{get, post, put, delete},
    Router,
};
use tokio::net::TcpListener;
use tracing_subscriber;

mod models;
mod error;
mod db;
mod handlers;

use handlers::*;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Initialize tracing (logging)
    tracing_subscriber::fmt::init();

    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Get database URL from environment
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    // Initialize database pool
    let pool = db::init_db(&database_url)
        .await
        .expect("Failed to initialize database");

    // Build the router with all routes
    let cors = CorsLayer::permissive();
    
    let app = Router::new()
        .route("/tasks", get(list_tasks))
        .route("/tasks", post(create_task))
        .route("/tasks/{id}", get(get_task))
        .route("/tasks/{id}", put(update_task))
        .route("/tasks/{id}", delete(delete_task))
        .with_state(pool)
        .layer(ServiceBuilder::new().layer(cors));

    // Create TCP listener on localhost:3000
    let listener = TcpListener::bind("127.0.0.1:3001")
        .await
        .expect("Failed to bind to port 3001");

    println!(" Server running on http://127.0.0.1:3001");

    // Start the server
    axum::serve(listener, app)
        .await
        .expect("Server error");
}