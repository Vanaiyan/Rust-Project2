use anyhow::{Result, anyhow};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::repository::product_repository::ProductRepository;
use crate::models::product_model::{CreateProductModel, ProductModel};

pub struct ProductService;

impl ProductService {
    pub async fn create_product(
        db: &DatabaseConnection, 
        product_data: CreateProductModel
    ) -> Result<ProductModel> {
        // Add any business logic validation here
        if product_data.Name.is_empty() {
            return Err(anyhow!("Product name cannot be empty"));
        }

        ProductRepository::create(db, product_data).await
    }

    pub async fn get_all_products(db: &DatabaseConnection) -> Result<Vec<ProductModel>> {
        ProductRepository::find_all(db).await
    }

    pub async fn get_product_by_uuid(
        db: &DatabaseConnection, 
        uuid: Uuid
    ) -> Result<ProductModel> {
        ProductRepository::find_by_uuid(db, uuid)
            .await?
            .ok_or_else(|| anyhow!("Product not found"))
    }

    pub async fn update_product(
        db: &DatabaseConnection, 
        uuid: Uuid, 
        update_data: CreateProductModel
    ) -> Result<ProductModel> {
        // Add any business logic validation here
        if update_data.Name.is_empty() {
            return Err(anyhow!("Product name cannot be empty"));
        }

        ProductRepository::update(db, uuid, update_data)
            .await?
            .ok_or_else(|| anyhow!("Product not found"))
    }

    pub async fn delete_product(
        db: &DatabaseConnection, 
        uuid: Uuid
    ) -> Result<()> {
        let deleted = ProductRepository::delete(db, uuid).await?;
        
        if !deleted {
            return Err(anyhow!("Product not found"));
        }

        Ok(())
    }
}