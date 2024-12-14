use sea_orm::{
    DatabaseConnection, 
    EntityTrait, 
    QueryFilter, 
    Set, 
    ActiveModelTrait
};
use entity::item::{self, Entity as ItemEntity};
use anyhow::Result;
use sea_orm::ColumnTrait;

use crate::models::item_model::ItemModel;


pub struct ItemRepository;

impl ItemRepository {
    pub async fn create(
        db: &DatabaseConnection, 
        item_data: ItemModel
    ) -> Result<ItemModel> {
        let new_item = item::ActiveModel {
            product_id: Set(item_data.ProductId),
            name: Set(item_data.Name.clone()),
            quantity: Set(item_data.Quantity),
            ..Default::default()
        };

        let inserted_item = new_item.insert(db).await?;

        Ok(ItemModel {
            id: Some(inserted_item.id),
            ProductId: inserted_item.product_id,
            Name: inserted_item.name,
            Quantity: inserted_item.quantity,
        })
    }

    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<ItemModel>> {
        let items = ItemEntity::find().all(db).await?;

        let response_items = items
            .into_iter()
            .map(|item| ItemModel {
                id: Some(item.id),
                ProductId: item.product_id,
                Name: item.name,
                Quantity: item.quantity,
            })
            .collect();

        Ok(response_items)
    }

    pub async fn find_by_id(
        db: &DatabaseConnection, 
        id: i32
    ) -> Result<Option<ItemModel>> {
        let item = ItemEntity::find()
            .filter(item::Column::Id.eq(id))
            .one(db)
            .await?;

        Ok(item.map(|item| ItemModel {
            id: Some(item.id),
            ProductId: item.product_id,
            Name: item.name,
            Quantity: item.quantity,
        }))
    }

    pub async fn update(
        db: &DatabaseConnection, 
        id: i32, 
        update_data: ItemModel
    ) -> Result<Option<ItemModel>> {
        // Find the existing item
        let item = ItemEntity::find_by_id(id).one(db).await?;

        match item {
            Some(existing_item) => {
                let mut active_model: item::ActiveModel = existing_item.into();

                // Update only the provided fields
                if let Some(name) = Some(update_data.Name) {
                    active_model.name = Set(name);
                }
                if update_data.Quantity != 0 {
                    active_model.quantity = Set(update_data.Quantity);
                }
                if update_data.ProductId != 0 {
                    active_model.product_id = Set(update_data.ProductId);
                }

                // Save the updated item
                let updated_item = active_model.update(db).await?;

                Ok(Some(ItemModel {
                    id: Some(updated_item.id),
                    ProductId: updated_item.product_id,
                    Name: updated_item.name,
                    Quantity: updated_item.quantity,
                }))
            }
            None => Ok(None)
        }
    }

    pub async fn delete(
        db: &DatabaseConnection, 
        id: i32
    ) -> Result<bool> {
        let result = ItemEntity::delete_by_id(id)
            .exec(db)
            .await?;

        Ok(result.rows_affected > 0)
    }
}