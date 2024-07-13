use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CategoryProduct::Table).to_owned())
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(CategoryProduct::Table)
                    .if_not_exists()
                    .primary_key(
                        Index::create()
                            .table(CategoryProduct::Table)
                            .col(CategoryProduct::ProductId)
                            .col(CategoryProduct::CategoryId),
                    )
                    .col(
                        ColumnDef::new(CategoryProduct::CategoryId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CategoryProduct::ProductId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(CategoryProduct::Table, CategoryProduct::ProductId)
                            .to(Product::Table, Product::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(CategoryProduct::Table, CategoryProduct::CategoryId)
                            .to(Category::Table, Category::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CategoryProduct::Table).to_owned())
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(CategoryProduct::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CategoryProduct::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(CategoryProduct::CategoryId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CategoryProduct::ProductId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_category_product_product")
                            .from(CategoryProduct::Table, CategoryProduct::ProductId)
                            .to(Product::Table, Product::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_category_product_category")
                            .from(CategoryProduct::Table, CategoryProduct::CategoryId)
                            .to(Category::Table, Category::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(iden = "category_product")]
enum CategoryProduct {
    Table,
    Id,
    CategoryId,
    ProductId,
}

#[derive(DeriveIden)]
enum Category {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Product {
    Table,
    Id,
}
