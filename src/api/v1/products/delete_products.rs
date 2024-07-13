use actix_web::{
    web::{Data, Path},
    HttpResponse, Responder,
};
use validator::Validate;

use crate::{
    api::{errors::ApiError, v1::products::dto::DeleteProductsDto},
    services::product::ProductService,
};

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
