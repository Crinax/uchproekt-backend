use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FieldProduct::Table).to_owned())
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(FieldProduct::Table)
                    .if_not_exists()
                    .primary_key(
                        Index::create()
                            .table(FieldProduct::Table)
                            .col(FieldProduct::ProductId)
                            .col(FieldProduct::FieldId),
                    )
                    .col(ColumnDef::new(FieldProduct::ProductId).integer().not_null())
                    .col(ColumnDef::new(FieldProduct::FieldId).integer().not_null())
                    .col(ColumnDef::new(FieldProduct::Value).text().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(FieldProduct::Table, FieldProduct::ProductId)
                            .to(Product::Table, Product::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(FieldProduct::Table, FieldProduct::FieldId)
                            .to(Field::Table, Field::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FieldProduct::Table).to_owned())
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(FieldProduct::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(FieldProduct::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(FieldProduct::ProductId).integer().not_null())
                    .col(ColumnDef::new(FieldProduct::FieldId).integer().not_null())
                    .col(ColumnDef::new(FieldProduct::Value).text().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_field_product_product")
                            .from(FieldProduct::Table, FieldProduct::ProductId)
                            .to(Product::Table, Product::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_field_product_category")
                            .from(FieldProduct::Table, FieldProduct::FieldId)
                            .to(Field::Table, Field::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(iden = "field_product")]
enum FieldProduct {
    Table,
    Id,
    ProductId,
    FieldId,
    Value,
}

#[derive(DeriveIden)]
enum Product {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Field {
    Table,
    Id,
}
