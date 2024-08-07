use rust_decimal::Decimal;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Clone, Validate)]
pub struct UpdateCreateCompanyServiceDto {
    #[validate(length(min = 1))]
    pub name: String,
    pub price: Decimal,
}
