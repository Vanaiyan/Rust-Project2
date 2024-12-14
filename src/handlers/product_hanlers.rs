use axum::{
    extract::{Path, Extension},
    http::StatusCode,
    response::IntoResponse,
    Json
};
use sea_orm::DatabaseConnection;
use uuid::Uuid;
use serde::Serialize;


use crate::service::product_service::ProductService;
use crate::models::product_model::{CreateProductModel, ProductModel};


#[derive(Serialize)]
#[serde(untagged)]
pub enum ProductResponse {
    Success(ProductModel),
    SuccessList(Vec<ProductModel>),
    Error { message: String },
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum DeleteResponse {
    Success { message: String },
    Error { message: String },
}



pub async fn create_product(
    Extension(db): Extension<DatabaseConnection>,
    Json(product_data): Json<CreateProductModel>
) -> impl IntoResponse {
    match ProductService::create_product(&db, product_data).await {
        Ok(product) => (StatusCode::CREATED, Json(ProductResponse::Success(product))),
        Err(e) => (
            StatusCode::BAD_REQUEST, 
            Json(ProductResponse::Error { message: e.to_string() })
        )
    }
}

pub async fn get_all_products(
    Extension(db): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    match ProductService::get_all_products(&db).await {
        Ok(products) => (
            StatusCode::OK, 
            Json(ProductResponse::SuccessList(products))
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR, 
            Json(ProductResponse::Error { message: e.to_string() })
        )
    }
}

pub async fn get_product_by_uuid(
    Extension(db): Extension<DatabaseConnection>,
    Path(uuid): Path<Uuid>,
) -> impl IntoResponse {
    match ProductService::get_product_by_uuid(&db, uuid).await {
        Ok(product) => (StatusCode::OK, Json(ProductResponse::Success(product))),
        Err(e) => (
            StatusCode::NOT_FOUND, 
            Json(ProductResponse::Error { message: e.to_string() })
        )
    }
}

pub async fn update_product(
    Extension(db): Extension<DatabaseConnection>,
    Path(uuid): Path<Uuid>,
    Json(update_data): Json<CreateProductModel>,
) -> impl IntoResponse {
    match ProductService::update_product(&db, uuid, update_data).await {
        Ok(product) => (StatusCode::OK, Json(ProductResponse::Success(product))),
        Err(e) => (
            StatusCode::NOT_FOUND, 
            Json(ProductResponse::Error { message: e.to_string() })
        )
    }
}

pub async fn delete_product(
    Extension(db): Extension<DatabaseConnection>,
    Path(uuid): Path<Uuid>,
) -> impl IntoResponse {
    match ProductService::delete_product(&db, uuid).await {
        Ok(_) => (
            StatusCode::OK, 
            Json(DeleteResponse::Success { 
                message: "Product deleted successfully".to_string() 
            })
        ),
        Err(e) => (
            StatusCode::NOT_FOUND, 
            Json(DeleteResponse::Error { message: e.to_string() })
        )
    }
}