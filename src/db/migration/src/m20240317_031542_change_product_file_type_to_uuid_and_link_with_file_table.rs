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
                    .drop_column(Product::Photo)
                    .add_column(
                        ColumnDef::new(Product::Photo)
                            .null()
                            .uuid()
                    )
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_product_file")
                            .from_tbl(Product::Table)
                            .from_col(Product::Photo)
                            .to_tbl(File::Table)
                            .to_col(File::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Product::Table)
                    .drop_foreign_key(Alias::new("fk_product_file"))
                    .modify_column(
                        ColumnDef::new(Product::Photo)
                            .string()
                            .default("")
                            .not_null()
                    )
                    .to_owned()
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Product {
    Table,
    Photo,
}

#[derive(DeriveIden)]
enum File {
    Table,
    Id,
}
