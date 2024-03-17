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
                    .modify_column(
                        ColumnDef::new(Product::Photo)
                            .null()
                    )
                    .modify_column(
                        ColumnDef::new(Product::Photo)
                            .extra("ALTER COLUMN \"photo\" TYPE uuid USING NULLIF(\"photo\", '')")
                    )
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_product_file")
                            .from_tbl(Product::Table)
                            .from_col(Product::Id)
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
                    )
                    .to_owned()
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Product {
    Table,
    Id,
    Photo,
}

#[derive(DeriveIden)]
enum File {
    Table,
    Id,
}
