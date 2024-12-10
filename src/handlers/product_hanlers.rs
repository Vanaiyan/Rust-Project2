use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use chrono::Utc;
use entity::product::{self, ActiveModel};
use sea_orm::{ActiveModelTrait, Set};
use uuid::Uuid;
use crate::models::product_model::{CreateProductModel, ProductModel};
use entity::product::Entity as ProductEntity;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter};
use ::serde::Serialize;
use sea_orm::ColumnTrait;
use axum::extract::Path;


pub async fn create_product(
    Extension(db): Extension<DatabaseConnection>,
    Json(product_data): Json<CreateProductModel>
) -> impl IntoResponse {

    // Create a new ActiveModel to insert the product into the database
    let product_model: ActiveModel = product::ActiveModel {
        uuid: Set(Uuid::new_v4()),
        name: Set(product_data.Name.to_owned()),
        description: Set(product_data.Description.to_owned()),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    // Insert the product into the database
    let inserted_product = product_model.insert(&db).await.unwrap();

    // Close the database connection
    db.close().await.unwrap();

    // Create a ProductModel instance to return
    let response_product = ProductModel {
        uuid: inserted_product.uuid,
        Name: inserted_product.name,
        Description: inserted_product.description,
        Created_at: inserted_product.created_at,
    };

    // Return the inserted product details as a response with StatusCode::CREATED
    (StatusCode::CREATED, Json(response_product))
}



// pub async fn get_all_products(
//     Extension(db): Extension<DatabaseConnection>,
// ) -> impl IntoResponse {
    
//     // Fetch all products from the database
//     let products = ProductEntity::find().all(&db).await.unwrap();

//     // Map the results to a vector of ProductModel
//     let response_products: Vec<ProductModel> = products
//         .into_iter()
//         .map(|p| ProductModel {
//             uuid: p.uuid,
//             Name: p.name,
//             Description: p.description,
//             Created_at: p.created_at,
//         })
//         .collect();

//     // Return the list of products as JSON with StatusCode::OK
//     (StatusCode::OK, Json(response_products))
// }
pub async fn get_all_products(
    Extension(db): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    match ProductEntity::find().all(&db).await {
        Ok(products) => {
            let response_products: Vec<ProductModel> = products
                .into_iter()
                .map(|p| ProductModel {
                    uuid: p.uuid,
                    Name: p.name,
                    Description: p.description,
                    Created_at: p.created_at,
                })
                .collect();
            (StatusCode::OK, Json(response_products)).into_response()
        }
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ProductResponse::Error {
                    message: "Failed to fetch products".to_string(),
                }),
            )
                .into_response()
        }
    }
}



#[derive(Serialize)]
#[serde(untagged)]
pub enum ProductResponse {
    Success(ProductModel),
    Error { message: String },
}

pub async fn get_product_by_uuid(
    Extension(db): Extension<DatabaseConnection>,
    Path(uuid): Path<Uuid>,
) -> impl IntoResponse {

    let product = ProductEntity::find()
        .filter(product::Column::Uuid.eq(uuid))
        .one(&db)
        .await
        .unwrap();

    match product {
        Some(p) => {
            let response_product = ProductModel {
                uuid: p.uuid,
                Name: p.name,
                Description: p.description,
                Created_at: p.created_at,
            };
            (StatusCode::OK, Json(ProductResponse::Success(response_product)))
        }
        None => (
            StatusCode::NOT_FOUND,
            Json(ProductResponse::Error {
                message: "Product not found".to_string(),
            }),
        ),
    }
}




#[derive(Serialize)]
#[serde(untagged)]
pub enum DeleteResponse {
    Success { message: String },
    Error { message: String },
}

pub async fn delete_product(
    Extension(db): Extension<DatabaseConnection>,
    Path(uuid): Path<Uuid>,
) -> impl IntoResponse {
    
    // Attempt to delete the product
    let result = ProductEntity::delete_many()
        .filter(entity::product::Column::Uuid.eq(uuid))
        .exec(&db)
        .await;

    match result {
        Ok(delete_result) => {
            if delete_result.rows_affected > 0 {
                // Product was successfully deleted
                (
                    StatusCode::OK,
                    Json(DeleteResponse::Success {
                        message: "Product deleted successfully".to_string(),
                    }),
                )
            } else {
                // Product not found
                (
                    StatusCode::NOT_FOUND,
                    Json(DeleteResponse::Error {
                        message: "Product not found".to_string(),
                    }),
                )
            }
        }
        Err(e) => {
            // Handle unexpected database error
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(DeleteResponse::Error {
                    message: format!("Failed to delete product: {}", e),
                }),
            )
        }
    }
}

pub async fn update_product(
    Extension(db): Extension<DatabaseConnection>,
    Path(uuid): Path<Uuid>,
    Json(update_data): Json<CreateProductModel>,
) -> impl IntoResponse {
    // Attempt to find the product by UUID
    let product = ProductEntity::find()
        .filter(product::Column::Uuid.eq(uuid))
        .one(&db)
        .await
        .unwrap();

    match product {
        Some(existing_product) => {
            // Create an ActiveModel for updating
            let mut product_model: ActiveModel = existing_product.into();

            // Update fields
            product_model.name = Set(update_data.Name.to_owned());
            product_model.description = Set(update_data.Description.to_owned());
            product_model.created_at = Set(Utc::now().naive_utc());

            // Save the updated product
            let updated_product = product_model.update(&db).await.unwrap();

            // Map to response model
            let response_product = ProductModel {
                uuid: updated_product.uuid,
                Name: updated_product.name,
                Description: updated_product.description,
                Created_at: updated_product.created_at,
            };

            // Return the updated product
            (StatusCode::OK, Json(ProductResponse::Success(response_product)))
        }
        None => (
            // Product not found
            StatusCode::NOT_FOUND,
            Json(ProductResponse::Error {
                message: "Product not found".to_string(),
            }),
        ),
    }
}