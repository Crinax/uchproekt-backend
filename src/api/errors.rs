use super::JsonMessage;
use actix_web::HttpResponse;

pub fn invalid_data() -> HttpResponse {
    HttpResponse::BadRequest().json(JsonMessage {
        message: "invalid_data",
    })
}
