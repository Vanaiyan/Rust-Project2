use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(Product::Table)
                    .if_not_exists()
                    .col(pk_auto(Product::Id))
                    .col(ColumnDef::new(Product::Uuid).uuid().unique_key().not_null())
                    .col(string(Product::Name))
                    .col(string(Product::Description))
                    .col(date_time(Product::Created_at))
                    .to_owned(),
            )
            .await?;

            manager
            .create_table(
                Table::create()
                    .table(Item::Table)
                    .if_not_exists()
                    .col(pk_auto(Item::Id))
                    .col(string(Item::Name))
                    .col(integer(Item::ProductId))
                    .col(integer(Item::Quantity))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_item_product")
                            .from(Item::Table, Item::ProductId)
                            .to(Product::Table, Product::Id),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(Product::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Item::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Product {
    Table,
    Id,
    Uuid,
    Name,
    Description,
    Created_at
}

#[derive(DeriveIden)]
enum Item {
    Table,
    Id,
    ProductId,
    Name,
    Quantity

}
