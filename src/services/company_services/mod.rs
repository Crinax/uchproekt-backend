pub mod dto;

use dto::{
    CompanyServiceIdSerializable, CompanyServiceSerializable, GetCreateCompanyServicesError,
    UpdateRemoveCompanyServiceError,
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

    pub async fn create(
        &self,
        name: &str,
        price: Decimal,
    ) -> Result<CompanyServiceIdSerializable, GetCreateCompanyServicesError> {
        let model = service::ActiveModel {
            name: Set(name.to_string()),
            price: Set(price),
            ..Default::default()
        };

        service::Entity::insert(model)
            .exec(&self.db)
            .await
            .map(|result| CompanyServiceIdSerializable {
                id: result.last_insert_id as u32,
            })
            .map_err(|_| GetCreateCompanyServicesError::InternalError)
    }

    pub async fn get_all(
        &self,
    ) -> Result<Vec<CompanyServiceSerializable>, GetCreateCompanyServicesError> {
        service::Entity::find()
            .all(&self.db)
            .await
            .map(|result| result.into_iter().map(|s| s.into()).collect())
            .map_err(|_| GetCreateCompanyServicesError::InternalError)
    }

    pub async fn update(
        &self,
        id: u32,
        name: &str,
        price: Decimal,
    ) -> Result<CompanyServiceIdSerializable, UpdateRemoveCompanyServiceError> {
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
            .map_err(|err| match err {
                sea_orm::DbErr::RecordNotFound(_) => UpdateRemoveCompanyServiceError::NotFound,
                _ => UpdateRemoveCompanyServiceError::InternalError,
            })
    }

    pub async fn delete(
        &self,
        id: u32,
    ) -> Result<CompanyServiceIdSerializable, UpdateRemoveCompanyServiceError> {
        service::Entity::delete_by_id(id as i32)
            .exec(&self.db)
            .await
            .map(|_| CompanyServiceIdSerializable { id })
            .map_err(|err| match err {
                sea_orm::DbErr::RecordNotFound(_) => UpdateRemoveCompanyServiceError::NotFound,
                _ => UpdateRemoveCompanyServiceError::InternalError,
            })
    }
}
