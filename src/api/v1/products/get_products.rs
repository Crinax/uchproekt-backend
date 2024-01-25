use crate::{api::errors::ApiError, services::product::{ProductService, ProductServiceErr}};
use actix_web::{get, web::{Data, Path}, HttpResponse, Responder};

#[get("")]
pub(super) async fn get_products(products_service: Data<ProductService>) -> impl Responder {
    let products = products_service.all().await;

    if products.is_err() {
        return ApiError::internal_error();
    }

    products.map(|data| HttpResponse::Ok().json(data)).unwrap()
}

#[get(":id")]
pub(super) async fn get_concreate_product(
    path_id: Path<u32>,
    product_service: Data<ProductService>
) -> impl Responder {
    let product = product_service.get(path_id.into_inner()).await
        .map(|data| HttpResponse::Ok().json(data))
        .map_err(|err| match err {
            ProductServiceErr::Internal => ApiError::internal_error(),
            ProductServiceErr::NotFound => ApiError::not_found()
        });

    if product.is_err() {
        return product.err().unwrap();
    }

    product.unwrap()
}
