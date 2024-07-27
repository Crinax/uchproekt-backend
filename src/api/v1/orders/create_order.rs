use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use validator::Validate;

use crate::{
    api::{errors::ApiError, v1::orders::dto::CreateOrderDto},
    services::order::{OrderInsertionErr, OrderService},
};

#[post("")]
pub(super) async fn create_order(
    order_service: Data<OrderService>,
    body: Json<CreateOrderDto>,
) -> impl Responder {
    if body.validate().is_err() {
        return ApiError::invalid_data();
    }

    let order = order_service
        .create(
            body.0.name,
            body.0.surname,
            body.0.phone,
            body.0.address,
            body.0.products,
        )
        .await
        .map_err(|err| {
            if let OrderInsertionErr::ProductNotFound(ids) = err {
                return ApiError::not_found_ids(ids);
            }

            ApiError::internal_error()
        })
        .map(|res| HttpResponse::Ok().json(res));

    if let Err(err) = order {
        return err;
    }

    order.unwrap()
}
