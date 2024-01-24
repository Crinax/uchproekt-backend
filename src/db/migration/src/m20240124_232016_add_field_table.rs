use sea_orm_migration::{prelude::*, sea_orm::EnumIter};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Field::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Field::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Field::Name).string().not_null())
                    .col(ColumnDef::new(Field::Type).custom(FieldType::Table).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Field::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Field {
    Table,
    Id,
    Name,
    Type,
}

#[derive(DeriveIden, EnumIter)]
enum FieldType {
    #[sea_orm(iden = "field_type")]
    Table,
    #[sea_orm(iden = "integer")]
    Integer,
    #[sea_orm(iden = "string")]
    Str,
}
