mod routes;
mod models;
mod handlers;
pub mod service;
pub mod repository;

use axum::{Extension, Router};
use sea_orm::Database;
use dotenv::dotenv;
use std::env;


pub async fn app() {

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(database_url).await.expect("Failed to connect to db");

    let app = Router::new()
        .merge(routes::item_routers::item_routes())
        .merge(routes::product_routes::product_routes())
        .layer(Extension(db));

    // Use PORT environment variable, default to 3002 if not set
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);
    
    let tcp_listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Server listening on {}", addr);
    axum::serve(tcp_listener, app).await.unwrap();
}