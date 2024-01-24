mod authorize;
mod refresh_tokens;
mod logout;
mod dto;

use actix_web::web;
use serde::Serialize;

#[derive(Serialize)]
struct AuthDataResult {
    access_token: String,
    expires: usize,
}

pub(super) fn configure() -> impl Fn(&mut web::ServiceConfig) {
    move |cfg| {
        cfg
            .service(authorize::authorize)
            .service(refresh_tokens::refresh_tokens)
            .service(logout::logout);
    }
}
