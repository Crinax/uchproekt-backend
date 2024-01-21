use actix_web::HttpResponse;
use super::JsonMessage;

pub fn invalid_data() -> HttpResponse {
    HttpResponse::BadRequest().json(JsonMessage {
        message: "invalid_data",
    })
}
