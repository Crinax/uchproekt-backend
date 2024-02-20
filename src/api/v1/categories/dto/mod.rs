use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Debug, Clone)]
pub struct DeleteCategoriesDto {
    pub categories: Vec<u32>,
}

#[derive(Deserialize, Validate, Debug, Clone)]
pub struct CreateCategoryDto {
    #[validate(length(min = 3, max = 32))]
    pub name: String,

    pub parent_id: Option<u32>
}
