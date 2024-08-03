use actix_web::{get, web::Data, HttpResponse, Responder};

use crate::{api::errors::ApiError, services::company_services::CompanyServicesService};

#[get("")]
pub(super) async fn get_company_services(service: Data<CompanyServicesService>) -> impl Responder {
    let result = service
        .get_all()
        .await
        .map(|result| HttpResponse::Ok().json(result))
        .map_err(|_| ApiError::internal_error());

    if result.is_err() {
        result.unwrap_err()
    } else {
        result.unwrap()
    }
}
