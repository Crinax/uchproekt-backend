use actix_web::{
    web::{Data, Json},
    HttpResponse, Responder,
};
use rust_decimal::Decimal;
use validator::Validate;

use crate::{api::errors::ApiError, services::product::ProductService};

use super::dto::CreateProductsDto;

pub(super) async fn create_product(
    data: Json<CreateProductsDto>,
    product_service: Data<ProductService>,
) -> impl Responder {
    if data.validate().is_err() {
        return ApiError::invalid_data();
    }

    if data.0.price < Decimal::new(0, 0) {
        return ApiError::invalid_data();
    }

    let result = product_service
        .create(
            data.0.name,
            data.0.price,
            data.0.article,
            data.0.description,
            data.0.photo,
            data.0.fields,
            data.0.category_id,
        )
        .await
        .map(|value| HttpResponse::Ok().json(value))
        .map_err(|_| ApiError::internal_error());

    if result.is_err() {
        return result.err().unwrap();
    }

    result.unwrap()
}
