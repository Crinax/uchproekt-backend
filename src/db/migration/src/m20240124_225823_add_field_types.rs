use sea_orm_migration::{
    prelude::*,
    sea_orm::{EnumIter, Iterable},
    sea_query::extension::postgres::Type
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(FieldType::Table)
                    .values(FieldType::iter().skip(1))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_type(Type::drop().name(FieldType::Table).to_owned())
            .await
    }
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
