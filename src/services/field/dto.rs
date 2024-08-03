use super::field_type::FieldType;
use entity::field::Model as FieldModel;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct FieldSerializable {
    pub id: i32,
    pub name: String,
    pub r#type: FieldType,
}

#[derive(Clone, Debug, Serialize)]
pub struct FieldId {
    pub(super) id: u32,
}

#[derive(Clone, Debug, Serialize)]
pub enum FieldCreateError {
    #[serde(rename = "already_exists")]
    AlreadyExists,

    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Clone, Debug, Serialize)]
pub enum FieldGetRemoveError {
    #[serde(rename = "not_found")]
    NotFound,

    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Clone, Debug, Serialize)]
pub enum FieldUpdateError {
    #[serde(rename = "not_found")]
    NotFound,

    #[serde(rename = "unknown")]
    Unknown,
}

impl From<FieldModel> for FieldSerializable {
    fn from(model: FieldModel) -> Self {
        Self {
            id: model.id,
            name: model.name,
            r#type: model.r#type.into(),
        }
    }
}
