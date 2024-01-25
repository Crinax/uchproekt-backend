use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Debug, Clone)]
pub struct DeleteProductsDto {
    pub products: Vec<u32>,
}
