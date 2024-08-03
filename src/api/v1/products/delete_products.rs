use actix_web::{
    web::{Data, Path},
    HttpResponse, Responder,
};
use validator::Validate;

use crate::{
    api::{errors::ApiError, v1::products::dto::DeleteProductsDto},
    services::product::{ProductService, ProductServiceErr},
};

pub(super) async fn delete_field_from_product(
    product_id: Path<u32>,
    field_id: Path<u32>,
    service: Data<ProductService>,
) -> impl Responder {
    let result = service
        .remove_field_from_product(product_id.into_inner(), field_id.into_inner())
        .await
        .map_err(|e| match e {
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

pub(super) async fn delete_products(
    data: Path<DeleteProductsDto>,
    product_service: Data<ProductService>,
) -> impl Responder {
    if data.validate().is_err() {
        return ApiError::invalid_data();
    }

    let deletion_result = product_service
        .delete(&[data.id])
        .await
        .map(|data| HttpResponse::Ok().json(data));

    if deletion_result.is_err() {
        return ApiError::internal_error();
    }

    deletion_result.unwrap()
}
