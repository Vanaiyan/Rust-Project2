use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, Set, ActiveModelTrait};
use entity::product::{self, Entity as ProductEntity, ActiveModel};
use uuid::Uuid;
use chrono::Utc;
use anyhow::Result;
use sea_orm::ColumnTrait;

use crate::models::product_model::{CreateProductModel, ProductModel};


pub struct ProductRepository;

impl ProductRepository {
    pub async fn create(db: &DatabaseConnection, product_data: CreateProductModel) -> Result<ProductModel> {
        let product_model: ActiveModel = product::ActiveModel {
            uuid: Set(Uuid::new_v4()),
            name: Set(product_data.Name.to_owned()),
            description: Set(product_data.Description.to_owned()),
            created_at: Set(Utc::now().naive_utc()),
            ..Default::default()
        };

        let inserted_product = product_model.insert(db).await?;

        Ok(ProductModel {
            uuid: inserted_product.uuid,
            Name: inserted_product.name,
            Description: inserted_product.description,
            Created_at: inserted_product.created_at,
        })
    }

    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<ProductModel>> {
        let products = ProductEntity::find().all(db).await?;

        let response_products = products
            .into_iter()
            .map(|p| ProductModel {
                uuid: p.uuid,
                Name: p.name,
                Description: p.description,
                Created_at: p.created_at,
            })
            .collect();

        Ok(response_products)
    }

    pub async fn find_by_uuid(db: &DatabaseConnection, uuid: Uuid) -> Result<Option<ProductModel>> {
        let product = ProductEntity::find()
            .filter(product::Column::Uuid.eq(uuid))
            .one(db)
            .await?;

        Ok(product.map(|p| ProductModel {
            uuid: p.uuid,
            Name: p.name,
            Description: p.description,
            Created_at: p.created_at,
        }))
    }

    pub async fn update(
        db: &DatabaseConnection, 
        uuid: Uuid, 
        update_data: CreateProductModel
    ) -> Result<Option<ProductModel>> {
        let product = ProductEntity::find()
            .filter(product::Column::Uuid.eq(uuid))
            .one(db)
            .await?;

        match product {
            Some(existing_product) => {
                let mut product_model: ActiveModel = existing_product.into();

                product_model.name = Set(update_data.Name.to_owned());
                product_model.description = Set(update_data.Description.to_owned());
                product_model.created_at = Set(Utc::now().naive_utc());

                let updated_product = product_model.update(db).await?;

                Ok(Some(ProductModel {
                    uuid: updated_product.uuid,
                    Name: updated_product.name,
                    Description: updated_product.description,
                    Created_at: updated_product.created_at,
                }))
            }
            None => Ok(None)
        }
    }

    pub async fn delete(db: &DatabaseConnection, uuid: Uuid) -> Result<bool> {
        let result = ProductEntity::delete_many()
            .filter(entity::product::Column::Uuid.eq(uuid))
            .exec(db)
            .await?;

        Ok(result.rows_affected > 0)
    }
}