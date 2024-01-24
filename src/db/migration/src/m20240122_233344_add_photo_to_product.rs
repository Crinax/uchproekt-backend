use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Product::Table)
                    .add_column(
                        ColumnDef::new(Product::Photo)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .modify_column(
                        ColumnDef::new(Product::Description)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .drop_column(Product::Photo)
                    .modify_column(ColumnDef::new(Product::Description).string().not_null())
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Product {
    Table,
    Description,
    Photo,
}
