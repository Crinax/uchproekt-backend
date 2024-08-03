use actix_web::{
    web::{self, Data},
    HttpResponse, Responder,
};

use crate::{
    api::errors::ApiError,
    services::field::{dto::FieldGetRemoveError, FieldService},
};

pub(super) async fn delete_field(
    service: Data<FieldService>,
    field_id: web::Path<u32>,
) -> impl Responder {
    let result = service
        .remove(field_id.into_inner())
        .await
        .map_err(|e| match e {
            FieldGetRemoveError::NotFound => ApiError::not_found(),
            _ => ApiError::internal_error(),
        })
        .map(|result| HttpResponse::Ok().json(result));

    if result.is_err() {
        result.unwrap_err()
    } else {
        result.unwrap()
    }
}
