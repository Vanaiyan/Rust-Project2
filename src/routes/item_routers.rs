use crate::handlers::item_handlers::{
    create_item, 
    get_all_items, 
    get_item_by_id, 
    delete_item, 
    update_item
};
use axum::{routing::{delete, get, post, put}, Router};

pub fn item_routes() -> Router {
    Router::new()
        .route("/api/item", post(create_item))
        .route("/api/get_all_items", get(get_all_items))
        .route("/api/get_item/:id", get(get_item_by_id))
        .route("/api/delete_item/:id", delete(delete_item))
        .route("/api/item/:id", put(update_item))
}