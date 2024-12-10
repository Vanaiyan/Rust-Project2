
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use entity::item::{self, Column};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, QueryFilter};
use crate::models::item_model::ItemModel;
use entity::item::Entity as ItemEntity;
use sea_orm::EntityTrait;
use axum::extract::Path;
use sea_orm::ColumnTrait;
use serde::Deserialize;


pub async fn create_item(
    Extension(db): Extension<DatabaseConnection>,
    Json(item_model): Json<ItemModel>,
) -> impl IntoResponse {
    let new_item = item::ActiveModel {
        product_id: Set(item_model.ProductId),
        name: Set(item_model.Name.clone()),
        quantity: Set(item_model.Quantity),
        ..Default::default()
    };

    match new_item.insert(&db).await {
        Ok(inserted_item) => (
            StatusCode::CREATED,
            Json(ItemModel {
                id: Some(inserted_item.id),
                ProductId: inserted_item.product_id,
                Name: inserted_item.name,
                Quantity: inserted_item.quantity,
            }),
        )
            .into_response(),
        Err(e) => {
            eprintln!("Error inserting item: {:?}", e); // Log the error for debugging
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to create item",
                    "details": e.to_string(),
                })),
            )
                .into_response()
        }
    }
}


pub async fn get_all_items(
    Extension(db): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    match ItemEntity::find().all(&db).await {
        Ok(items) => {
            let response_items: Vec<ItemModel> = items
                .into_iter()
                .map(|item| ItemModel {
                    id: Some(item.id),
                    ProductId: item.product_id,
                    Name: item.name,
                    Quantity: item.quantity,
                })
                .collect();

            (StatusCode::OK, Json(response_items)).into_response()
        }
        Err(e) => {
            eprintln!("Error fetching items: {:?}", e); // Log the error for debugging
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to fetch items",
                    "details": e.to_string(),
                })),
            )
                .into_response()
        }
    }
}


pub async fn get_item_by_id(
    Extension(db): Extension<DatabaseConnection>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match ItemEntity::find()
        .filter(Column::Id.eq(id))
        .one(&db)
        .await
    {
        Ok(Some(item)) => {
            // Convert the database item into the response model
            let response_item = ItemModel {
                id: Some(item.id),
                ProductId: item.product_id,
                Name: item.name,
                Quantity: item.quantity,
            };

            (StatusCode::OK, Json(response_item)).into_response()
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "Item not found",
                "id": id
            })),
        )
            .into_response(),
        Err(e) => {
            eprintln!("Error fetching item by ID: {:?}", e); // Log the error for debugging
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to fetch item",
                    "details": e.to_string(),
                })),
            )
                .into_response()
        }
    }
}


// Define the structure for the update payload
#[derive(Deserialize)]
pub struct UpdateItemPayload {
    pub name: Option<String>,
    pub quantity: Option<i32>,
}

pub async fn update_item_by_id(
    Extension(db): Extension<DatabaseConnection>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateItemPayload>,
) -> impl IntoResponse {
    // Find the existing item by ID
    match ItemEntity::find_by_id(id).one(&db).await {
        Ok(Some(existing_item)) => {
            // Convert the fetched item into an ActiveModel for updating
            let mut active_model: item::ActiveModel = existing_item.into();

            // Update only the provided fields
            if let Some(name) = payload.name {
                active_model.name = Set(name);
            }
            if let Some(quantity) = payload.quantity {
                active_model.quantity = Set(quantity);
            }

            // Save the updated ActiveModel
            match active_model.update(&db).await {
                Ok(updated_item) => (
                    StatusCode::OK,
                    Json(serde_json::json!({
                        "message": "Item updated successfully",
                        "updated_item": {
                            "id": updated_item.id,
                            "name": updated_item.name,
                            "quantity": updated_item.quantity,
                            "product_id": updated_item.product_id,
                        }
                    })),
                ),
                Err(e) => {
                    eprintln!("Error updating item: {:?}", e); // Log error
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({
                            "error": "Failed to update item",
                            "details": e.to_string(),
                        })),
                    )
                }
            }
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "Item not found",
                "id": id,
            })),
        ),
        Err(e) => {
            eprintln!("Error fetching item by ID: {:?}", e); // Log error
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to fetch item",
                    "details": e.to_string(),
                })),
            )
        }
    }
}


pub async fn delete_item_by_id(
    Extension(db): Extension<DatabaseConnection>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match ItemEntity::delete_by_id(id).exec(&db).await {
        Ok(delete_result) => {
            if delete_result.rows_affected > 0 {
                (
                    StatusCode::OK,
                    Json(serde_json::json!({
                        "message": format!("Item with id {} deleted successfully", id),
                    })),
                )
            } else {
                (
                    StatusCode::NOT_FOUND,
                    Json(serde_json::json!({
                        "error": "Item not found",
                        "id": id
                    })),
                )
            }
        }
        Err(e) => {
            eprintln!("Error deleting item by ID: {:?}", e); // Log the error for debugging
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to delete item",
                    "details": e.to_string(),
                })),
            )
        }
    }
}