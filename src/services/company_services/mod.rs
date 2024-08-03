mod dto;
use dto::{
    CompanyServiceIdSerializable, CompanyServiceSerializable, GetCompanyServicesError,
    UpdateCompanyServiceError,
};
use entity::service;
use rust_decimal::Decimal;
use sea_orm::{DatabaseConnection, EntityTrait, Set};

// Meme naming :D
pub struct CompanyServicesService {
    db: DatabaseConnection,
}

impl CompanyServicesService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_all(
        &self,
    ) -> Result<Vec<CompanyServiceSerializable>, GetCompanyServicesError> {
        service::Entity::find()
            .all(&self.db)
            .await
            .map(|result| result.into_iter().map(|s| s.into()).collect())
            .map_err(|_| GetCompanyServicesError::InternalError)
    }

    pub async fn update(
        &self,
        id: u32,
        name: &str,
        price: Decimal,
    ) -> Result<CompanyServiceIdSerializable, UpdateCompanyServiceError> {
        let model = service::ActiveModel {
            id: Set(id as i32),
            name: Set(name.to_string()),
            price: Set(price),
            ..Default::default()
        };
        service::Entity::update(model)
            .exec(&self.db)
            .await
            .map(|_| CompanyServiceIdSerializable { id })
            .map_err(|_| UpdateCompanyServiceError::InternalError)
    }

    pub async fn delete(
        &self,
        id: u32,
    ) -> Result<CompanyServiceIdSerializable, UpdateCompanyServiceError> {
        service::Entity::delete_by_id(id as i32)
            .exec(&self.db)
            .await
            .map(|_| CompanyServiceIdSerializable { id })
            .map_err(|_| UpdateCompanyServiceError::InternalError)
    }
}
