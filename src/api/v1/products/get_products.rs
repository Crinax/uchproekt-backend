use crate::{
    api::{errors::ApiError, v1::products::dto::PageAsQuery},
    services::product::{ProductService, ProductServiceErr},
};
use actix_web::{
    get,
    web::{Data, Path, Query},
    HttpResponse, Responder,
};
use validator::Validate;

#[get("")]
pub(super) async fn get_products(
    products_service: Data<ProductService>,
    query: Query<PageAsQuery>,
) -> impl Responder {
    if query.0.validate().is_err() {
        return ApiError::invalid_data();
    }

    let products = products_service.all(query.page - 1).await;

    if products.is_err() {
        return ApiError::internal_error();
    }

    products.map(|data| HttpResponse::Ok().json(data)).unwrap()
}

#[get("{id}")]
pub(super) async fn get_concreate_product(
    id: Path<u32>,
    product_service: Data<ProductService>,
) -> impl Responder {
    let product = product_service
        .get(id.into_inner())
        .await
        .map(|data| HttpResponse::Ok().json(data))
        .map_err(|err| match err {
            ProductServiceErr::Internal => ApiError::internal_error(),
            ProductServiceErr::NotFound => ApiError::not_found(),
        });

    if product.is_err() {
        return product.err().unwrap();
    }

    product.unwrap()
}
