use axum::{
    extract::{Path, Extension},
    http::StatusCode,
    response::IntoResponse,
    Json
};
use sea_orm::DatabaseConnection;
use serde::Serialize;


use crate::service::item_service::ItemService;
use crate::models::item_model::ItemModel;



#[derive(Serialize)]
#[serde(untagged)]
pub enum ItemResponse {
    Success(ItemModel),
    SuccessList(Vec<ItemModel>),
    Error { message: String },
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum DeleteResponse {
    Success { message: String },
    Error { message: String },
}



pub async fn create_item(
    Extension(db): Extension<DatabaseConnection>,
    Json(item_data): Json<ItemModel>
) -> impl IntoResponse {
    match ItemService::create_item(&db, item_data).await {
        Ok(item) => (StatusCode::CREATED, Json(ItemResponse::Success(item))),
        Err(e) => (
            StatusCode::BAD_REQUEST, 
            Json(ItemResponse::Error { message: e.to_string() })
        )
    }
}

pub async fn get_all_items(
    Extension(db): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    match ItemService::get_all_items(&db).await {
        Ok(items) => (StatusCode::OK, Json(ItemResponse::SuccessList(items))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR, 
            Json(ItemResponse::Error { message: e.to_string() })
        )
    }
}

pub async fn get_item_by_id(
    Extension(db): Extension<DatabaseConnection>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match ItemService::get_item_by_id(&db, id).await {
        Ok(item) => (StatusCode::OK, Json(ItemResponse::Success(item))),
        Err(e) => (
            StatusCode::NOT_FOUND, 
            Json(ItemResponse::Error { message: e.to_string() })
        )
    }
}

pub async fn update_item(
    Extension(db): Extension<DatabaseConnection>,
    Path(id): Path<i32>,
    Json(update_data): Json<ItemModel>,
) -> impl IntoResponse {
    match ItemService::update_item(&db, id, update_data).await {
        Ok(item) => (StatusCode::OK, Json(ItemResponse::Success(item))),
        Err(e) => (
            StatusCode::NOT_FOUND, 
            Json(ItemResponse::Error { message: e.to_string() })
        )
    }
}

pub async fn delete_item(
    Extension(db): Extension<DatabaseConnection>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match ItemService::delete_item(&db, id).await {
        Ok(_) => (
            StatusCode::OK, 
            Json(DeleteResponse::Success { 
                message: "Item deleted successfully".to_string() 
            })
        ),
        Err(e) => (
            StatusCode::NOT_FOUND, 
            Json(DeleteResponse::Error { message: e.to_string() })
        )
    }
}