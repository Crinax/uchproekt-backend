use actix_web::{
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use validator::Validate;

use crate::{
    api::errors::ApiError,
    services::company_services::{dto::UpdateRemoveCompanyServiceError, CompanyServicesService},
};

use super::dto::UpdateCompanyServiceDto;

pub(super) async fn update_service(
    id: Path<u32>,
    data: Json<UpdateCompanyServiceDto>,
    service: Data<CompanyServicesService>,
) -> impl Responder {
    if data.validate().is_err() {
        return ApiError::invalid_data();
    }

    let result = service
        .update(id.into_inner(), &data.name, data.price)
        .await
        .map(|result| HttpResponse::Ok().json(result))
        .map_err(|err| match err {
            UpdateRemoveCompanyServiceError::NotFound => ApiError::not_found(),
            _ => ApiError::internal_error(),
        });

    if result.is_err() {
        result.unwrap_err()
    } else {
        result.unwrap()
    }
}
