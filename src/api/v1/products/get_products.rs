use crate::{api::errors::ApiError, services::product::ProductService};
use actix_web::{get, web::Data, HttpResponse, Responder};

#[get("")]
pub(super) async fn get_products(products_service: Data<ProductService>) -> impl Responder {
    let products = products_service.all().await;

    if products.is_err() {
        return ApiError::internal_error();
    }

    products.map(|data| HttpResponse::Ok().json(data)).unwrap()
}
