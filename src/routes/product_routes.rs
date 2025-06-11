
use crate::handlers::product_hanlers::{
    create_product, 
    get_all_products, 
    get_product_by_uuid, 
    delete_product, 
    update_product};

use axum::{routing::{delete, get, post, put}, Router};

pub fn product_routes() -> Router {
    Router::new().route("/api/product", post(create_product))
                 .route("/api/get_products", get(get_all_products))
                 .route("/api/get_product/:uuid", get(get_product_by_uuid))
                 .route("/api/delete_product/:uuid", delete(delete_product))
                 .route("/api/product/:uuid", put(update_product))

}