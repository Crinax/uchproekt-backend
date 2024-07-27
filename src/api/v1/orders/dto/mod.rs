use serde::Deserialize;
use validator::Validate;

use crate::services::order::ProductWithQuantity;

#[derive(Deserialize, Validate, Debug, Clone)]
pub struct CreateOrderDto {
    #[validate(length(min = 1))]
    pub name: String,

    #[validate(length(min = 1))]
    pub surname: String,

    #[validate(length(min = 1))]
    pub phone: String,

    #[validate(length(min = 1))]
    pub address: String,

    pub products: Vec<ProductWithQuantity>,
}
