//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.11

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "order")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub surname: String,
    pub phone: String,
    pub address: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::products_in_order::Entity")]
    ProductsInOrder,
}

impl Related<super::products_in_order::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProductsInOrder.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
