use actix_web::{
    web::{Data, Path},
    HttpResponse, Responder,
};

use crate::{
    api::errors::ApiError,
    services::company_services::{dto::UpdateRemoveCompanyServiceError, CompanyServicesService},
};

pub(super) async fn delete_service(
    id: Path<u32>,
    service: Data<CompanyServicesService>,
) -> impl Responder {
    let result = service
        .delete(id.into_inner())
        .await
        .map_err(|err| match err {
            UpdateRemoveCompanyServiceError::NotFound => ApiError::not_found(),
            _ => ApiError::internal_error(),
        })
        .map(|result| HttpResponse::Ok().json(result));

    if result.is_err() {
        result.unwrap_err()
    } else {
        result.unwrap()
    }
}
