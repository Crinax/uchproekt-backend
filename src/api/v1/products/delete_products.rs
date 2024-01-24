use actix_web::{delete, HttpResponse, Responder};

use crate::api::JsonMessage;

#[delete("")]
pub(super) async fn delete_products() -> impl Responder {
    HttpResponse::Ok().json(JsonMessage { message: "ok" })
}
