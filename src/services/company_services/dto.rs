use rust_decimal::Decimal;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct CompanyServiceSerializable {
    pub id: u32,
    pub name: String,
    pub price: Decimal,
}

#[derive(Clone, Debug, Serialize)]
pub struct CompanyServiceIdSerializable {
    pub id: u32,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub enum GetCompanyServicesError {
    #[serde(rename = "internal_error")]
    InternalError,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub enum UpdateCompanyServiceError {
    #[serde(rename = "not_found")]
    NotFound,

    #[serde(rename = "internal_error")]
    InternalError,
}

impl From<entity::service::Model> for CompanyServiceSerializable {
    fn from(model: entity::service::Model) -> Self {
        Self {
            id: model.id as u32,
            name: model.name,
            price: model.price,
        }
    }
}
