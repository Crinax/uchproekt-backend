use actix_web::{
    web::{self, Data},
    HttpResponse, Responder,
};
use validator::Validate;

use crate::{
    api::errors::ApiError,
    services::field::{dto::FieldCreateError, FieldService},
};

use super::dto;

pub(super) async fn create_field(
    data: web::Json<dto::CreateFieldDto>,
    service: Data<FieldService>,
) -> impl Responder {
    if data.validate().is_err() {
        return ApiError::invalid_data();
    }

    let field = service
        .create(&data.name, &data.r#type)
        .await
        .map_err(|e| match e {
            FieldCreateError::AlreadyExists => ApiError::conflict(),
            _ => ApiError::internal_error(),
        })
        .map(|result| HttpResponse::Ok().json(result));

    if field.is_err() {
        field.unwrap_err()
    } else {
        field.unwrap()
    }
}
