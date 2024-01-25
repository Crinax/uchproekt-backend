use super::JsonMessage;
use actix_web::HttpResponse;

pub struct ApiError;

impl ApiError {
    pub fn invalid_data() -> HttpResponse {
        HttpResponse::BadRequest().json(JsonMessage {
            message: "invalid_data",
        })
    }

    pub fn internal_error() -> HttpResponse {
        HttpResponse::InternalServerError().json(JsonMessage {
            message: "internal_error"
        })
    }

    pub fn not_found() -> HttpResponse {
        HttpResponse::NotFound().json(JsonMessage {
            message: "not_found"
        })
    }
}
