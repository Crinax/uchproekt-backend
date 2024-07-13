use actix_web::{web::Data, HttpResponse, Responder};

use crate::{api::errors::ApiError, services::order::OrderService};

pub(super) async fn get_orders(order_service: Data<OrderService>) -> impl Responder {
    let categories = order_service.get_all().await;

    if categories.is_err() {
        return ApiError::internal_error();
    }

    categories
        .map(|data| HttpResponse::Ok().json(data))
        .unwrap()
}
