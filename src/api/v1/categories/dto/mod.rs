use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Debug, Clone)]
pub struct DeleteCategoriesDto {
    pub products: Vec<u32>,
}

#[derive(Deserialize, Validate, Debug, Clone)]
pub struct CreateCategory {
    #[validate(length(min = 3, max = 32))]
    pub name: String,
}
