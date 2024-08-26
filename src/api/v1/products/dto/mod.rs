use rust_decimal::Decimal;
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Validate, Debug, Clone)]
pub struct DeleteProductsDto {
    pub id: u32,
}

#[derive(Deserialize, Validate, Debug, Clone)]
pub struct FieldInProductDto {
    pub id: u32,
    #[validate(length(min = 1))]
    pub value: String,
}

#[derive(Deserialize, Validate, Debug, Clone)]
pub struct FieldInProductAddOrUpdate {
    #[validate(length(min = 1))]
    pub value: String,
}
#[derive(Deserialize, Validate, Debug, Clone)]
pub struct UpdateProductsDto {
    #[validate(length(min = 3))]
    pub name: String,

    pub price: Decimal,

    #[validate(length(min = 1))]
    pub article: String,

    #[validate(length(min = 1))]
    pub description: String,

    pub photo: Option<Uuid>,
}

#[derive(Deserialize, Validate, Debug, Clone)]
pub struct SearchProductsQuery {
    #[validate(range(min = 1))]
    #[serde(default = "default_page")]
    pub page: u64,

    #[serde(default)]
    pub query: String,
}

pub fn default_page() -> u64 {
    1
}

#[derive(Deserialize, Validate, Debug, Clone)]
pub struct CreateProductsDto {
    #[validate(length(min = 3))]
    pub name: String,

    pub price: Decimal,

    #[validate(length(min = 1))]
    pub article: String,

    #[validate(length(min = 1))]
    pub description: String,

    pub photo: Option<Uuid>,

    pub fields: Vec<FieldInProductDto>,
}
