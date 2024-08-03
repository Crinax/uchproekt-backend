pub mod dto;
pub mod field_type;

use dto::{FieldCreateError, FieldGetRemoveError, FieldId, FieldSerializable};
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
                id: field.last_insert_id as u32,
            })
    }

    pub async fn get_all(&self) -> Result<Vec<FieldSerializable>, FieldGetRemoveError> {
        Field::find()
            .all(&self.db)
            .await
            .map_err(|_| FieldGetRemoveError::Unknown)
            .map(|result| result.into_iter().map(FieldSerializable::from).collect())
    }

    pub async fn remove(&self, id: u32) -> Result<FieldId, FieldGetRemoveError> {
        Field::delete_by_id(id as i32)
            .exec(&self.db)
            .await
            .map_err(|err| match err {
                sea_orm::DbErr::RecordNotFound(_) => FieldGetRemoveError::NotFound,
                _ => FieldGetRemoveError::Unknown,
            })
            .map(|_| FieldId { id })
    }
}
