use actix_web::{
    web::{Data, Json},
    HttpResponse, Responder,
};

use crate::{
    api::{errors::ApiError, v1::orders::dto::CreateOrderDto},
    services::order::OrderService,
};

pub(super) async fn create_order(
    order_service: Data<OrderService>,
    body: Json<CreateOrderDto>,
) -> impl Responder {
    let order = order_service
        .create(
            body.0.name,
            body.0.surname,
            body.0.phone,
            body.0.address,
            body.0.products,
        )
        .await
        .map_err(|_| ApiError::internal_error())
        .map(|res| HttpResponse::Ok().json(res));

    if let Err(err) = order {
        return err;
    }

    order.unwrap()
}
