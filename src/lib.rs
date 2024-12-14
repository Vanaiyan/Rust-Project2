mod routes;
mod models;
mod handlers;

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

    let tcp_listener = tokio::net::TcpListener::bind("0.0.0.0:3002").await.unwrap();
    axum::serve(tcp_listener, app).await.unwrap();
}