pub mod errors;
mod middlewares;
mod v1;

use actix_web::web::{self, Data};
use serde::Serialize;

use crate::config::Config;

#[derive(Serialize)]
pub struct JsonMessage<'a> {
    pub message: &'a str,
}

#[derive(Serialize)]
pub struct JsonMessageWithContext<'a, T: Serialize> {
    pub message: &'a str,
    pub context: T,
}

pub(super) fn configure(config: Data<Config>) -> impl Fn(&mut web::ServiceConfig) {
    move |cfg| {
        cfg.service(web::scope("/v1").configure(v1::configure(config.clone())));
    }
}
