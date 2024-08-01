pub mod dto;
pub mod field_type;

use dto::{FieldCreateError, FieldGetError, FieldId, FieldSerializable};
use entity::field::{self, Entity as Field};
use field_type::FieldType;
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
}
