use crate::api::JsonMessage;
use actix_web::{get, Responder, HttpResponse};

#[get("")]
pub(self) async fn get_products() -> impl Responder {
    HttpResponse::Ok().json(JsonMessage {
        message: "ok"
    })
}
