use crate::utilities::serde_utils::Patch;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Debug, Clone)]
pub struct DeleteCategoriesDto {
    pub id: u32,
}

#[derive(Deserialize, Validate, Debug, Clone)]
pub struct CreateCategoryDto {
    #[validate(length(min = 3, max = 32))]
    pub name: String,

    pub parent_id: Option<u32>,
}

#[derive(Deserialize, Validate, Debug, Clone)]
pub struct UpdateCategoryDto {
    #[validate(length(min = 3, max = 32))]
    pub name: Option<String>,

    #[serde(default)]
    pub parent_id: Patch<u32>,
}
