use serde::Deserialize;
use validator::{Validate, ValidationError};

use crate::services::field::field_type::FieldType;

#[derive(Deserialize, Clone, Debug, Validate)]
pub struct CreateFieldDto {
    #[validate(length(min = 1))]
    pub name: String,

    #[validate(custom(function = "validate_field_type"))]
    pub r#type: String,
}

fn validate_field_type(value: &str) -> Result<(), ValidationError> {
    let result: FieldType = value.into();

    if result == FieldType::Unknown {
        return Err(ValidationError::new("invalid field type"));
    }

    Ok(())
}
