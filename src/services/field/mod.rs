pub mod dto;
pub mod field_type;

use dto::{FieldCreateError, FieldGetError, FieldId, FieldSerializable, FieldUpdateError};
use entity::{
    field::{self, Entity as Field},
    field_product::{self, Entity as FieldInProduct},
};
use field_type::FieldType;
use migration::OnConflict;
use sea_orm::{DatabaseConnection, EntityTrait, Set};

pub struct FieldService {
    db: DatabaseConnection,
}

impl FieldService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(&self, name: &str, field_type: &str) -> Result<FieldId, FieldCreateError> {
        let r#type: FieldType = field_type.into();
        let new_field = field::ActiveModel {
            name: Set(name.to_owned()),
            r#type: Set(r#type.into()),
            ..Default::default()
        };

        Field::insert(new_field)
            .exec(&self.db)
            .await
            .map_err(|err| match err {
                sea_orm::DbErr::RecordNotInserted => FieldCreateError::AlreadyExists,
                _ => FieldCreateError::Unknown,
            })
            .map(|field| FieldId {
                id: field.last_insert_id,
            })
    }

    pub async fn get(&self, id: i32) -> Result<FieldSerializable, FieldGetError> {
        Field::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|_| FieldGetError::Unknown)?
            .ok_or(FieldGetError::NotFound)
            .map(|field| dto::FieldSerializable {
                id: field.id,
                name: field.name,
                r#type: field.r#type.into(),
            })
    }

    pub async fn upsert_value(
        &self,
        product_id: i32,
        field_id: i32,
        value: &str,
    ) -> Result<(), FieldUpdateError> {
        FieldInProduct::insert(field_product::ActiveModel {
            field_id: Set(field_id),
            product_id: Set(product_id),
            value: Set(value.to_owned()),
        })
        .on_conflict(
            OnConflict::columns([
                field_product::Column::FieldId,
                field_product::Column::ProductId,
            ])
            .update_column(field_product::Column::Value)
            .to_owned(),
        )
        .exec(&self.db)
        .await
        .map_err(|_| FieldUpdateError::Unknown)
        .map(|_| ())
    }
}
