use crate::{api::JsonMessage, services::product::ProductService};
use actix_web::{get, Responder, HttpResponse, web::Data};

#[get("")]
pub(super) async fn get_products(products_service: Data<ProductService>) -> impl Responder {
    let products = products_service.all().await;

    if products.is_err() {
        return HttpResponse::InternalServerError().json(
            JsonMessage {
                message: "internal_error"
            }
        );
    }

    products
        .map(|data| HttpResponse::Ok().json(data))
        .unwrap()
}
