pub mod errors;
mod v1;

use actix_web::web;
use serde::Serialize;

#[derive(Serialize)]
pub struct JsonMessage<'a> {
    pub message: &'a str,
}

pub(super) fn configure() -> impl Fn(&mut web::ServiceConfig) {
    move |cfg| {
        cfg.service(web::scope("/v1").configure(v1::configure()));
    }
}
