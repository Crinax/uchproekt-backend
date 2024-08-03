use actix_web::{web::Data, HttpResponse, Responder};

use crate::{api::errors::ApiError, services::field::FieldService};

pub(super) async fn get_fields(service: Data<FieldService>) -> impl Responder {
    let result = service
        .get_all()
        .await
        .map_err(|e| match e {
            _ => ApiError::internal_error(),
        })
        .map(|fields| HttpResponse::Ok().json(fields));

    if result.is_err() {
        result.unwrap_err()
    } else {
        result.unwrap()
    }
}
