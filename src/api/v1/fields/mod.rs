mod create_fields;
pub mod dto;
mod get_fields;

use actix_web::web::{self, Data};

use crate::{api::middlewares::authenticate::JwtAuth, config::Config};

pub(super) fn configure(config: Data<Config>) -> impl Fn(&mut web::ServiceConfig) {
    move |cfg| {
        cfg.service(
            web::resource("")
                .wrap(JwtAuth::new(config.clone()))
                .post(create_fields::create_field)
                .get(get_fields::get_fields),
        );
    }
}
