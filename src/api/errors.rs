use crate::api::JsonMessageWithContext;

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
            message: "internal_error",
        })
    }

    pub fn not_found() -> HttpResponse {
        HttpResponse::NotFound().json(JsonMessage {
            message: "not_found",
        })
    }

    pub fn not_found_ids(ids: Vec<u32>) -> HttpResponse {
        HttpResponse::NotFound().json(JsonMessageWithContext {
            message: "not_found_ids",
            context: ids,
        })
    }

    pub fn conflict() -> HttpResponse {
        HttpResponse::Conflict().json(JsonMessage {
            message: "conflict",
        })
    }
}
