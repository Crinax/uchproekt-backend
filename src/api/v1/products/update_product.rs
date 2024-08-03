use actix_web::{
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use validator::Validate;

use crate::{
    api::errors::ApiError,
    services::product::{ProductService, ProductServiceErr},
};

use super::dto::CreateUpdateProductsDto;

pub(super) async fn update_product(
    id: Path<u32>,
    data: Json<CreateUpdateProductsDto>,
    service: Data<ProductService>,
) -> impl Responder {
    if data.validate().is_err() {
        return ApiError::invalid_data();
    }

    let result = service
        .update(
            id.into_inner(),
            &data.name,
            data.price,
            &data.article,
            &data.description,
            data.photo,
        )
        .await
        .map_err(|err| match err {
            ProductServiceErr::NotFound => ApiError::not_found(),
            _ => ApiError::internal_error(),
        })
        .map(|result| HttpResponse::Ok().json(result));

    if result.is_err() {
        return result.unwrap_err();
    } else {
        return result.unwrap();
    }
}
