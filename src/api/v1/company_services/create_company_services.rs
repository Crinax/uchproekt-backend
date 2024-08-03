use actix_web::{
    web::{Data, Json},
    HttpResponse, Responder,
};
use validator::Validate;

use crate::{api::errors::ApiError, services::company_services::CompanyServicesService};

use super::dto::UpdateCreateCompanyServiceDto;

pub(super) async fn create_service(
    data: Json<UpdateCreateCompanyServiceDto>,
    service: Data<CompanyServicesService>,
) -> impl Responder {
    if data.validate().is_err() {
        return ApiError::invalid_data();
    }

    let result = service
        .create(&data.name, data.price)
        .await
        .map_err(|_| ApiError::internal_error())
        .map(|result| HttpResponse::Ok().json(result));

    if result.is_err() {
        result.unwrap_err()
    } else {
        result.unwrap()
    }
}
