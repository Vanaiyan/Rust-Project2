use anyhow::{Result, anyhow};
use sea_orm::DatabaseConnection;

use crate::repository::item_repository::ItemRepository;
use crate::models::item_model::ItemModel;

pub struct ItemService;

impl ItemService {
    pub async fn create_item(
        db: &DatabaseConnection, 
        item_data: ItemModel
    ) -> Result<ItemModel> {
        // Validate input
        if item_data.Name.is_empty() {
            return Err(anyhow!("Item name cannot be empty"));
        }

        if item_data.ProductId == 0 {
            return Err(anyhow!("Invalid Product ID"));
        }

        // Call repository to create item
        ItemRepository::create(db, item_data).await
    }

    pub async fn get_all_items(
        db: &DatabaseConnection
    ) -> Result<Vec<ItemModel>> {
        ItemRepository::find_all(db).await
    }

    pub async fn get_item_by_id(
        db: &DatabaseConnection, 
        id: i32
    ) -> Result<ItemModel> {
        ItemRepository::find_by_id(db, id)
            .await?
            .ok_or_else(|| anyhow!("Item not found"))
    }

    pub async fn update_item(
        db: &DatabaseConnection, 
        id: i32, 
        update_data: ItemModel
    ) -> Result<ItemModel> {
        // Validate input
        if update_data.Name.is_empty() {
            return Err(anyhow!("Item name cannot be empty"));
        }

        ItemRepository::update(db, id, update_data)
            .await?
            .ok_or_else(|| anyhow!("Item not found"))
    }

    pub async fn delete_item(
        db: &DatabaseConnection, 
        id: i32
    ) -> Result<()> {
        let deleted = ItemRepository::delete(db, id).await?;
        
        if !deleted {
            return Err(anyhow!("Item not found"));
        }

        Ok(())
    }
}