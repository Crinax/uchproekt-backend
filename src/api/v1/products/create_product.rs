use actix_web::{web::{Data, Json}, HttpResponse, Responder};
use rust_decimal::Decimal;
use validator::Validate;

use crate::{api::{errors::ApiError, v1::products::dto::DeleteProductsDto, JsonMessage}, services::product::ProductService};

use super::dto::CreateProductsDto;

pub(super) async fn create_product(
    data: Json<CreateProductsDto>,
    product_service: Data<ProductService>
) -> impl Responder {
    if data.validate().is_err() {
        return ApiError::invalid_data();
    }

    if data.0.price < Decimal::new(0, 0) {
        return ApiError::invalid_data();
    }

    HttpResponse::Ok().json(JsonMessage { message: "ok" })
}
