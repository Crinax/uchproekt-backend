mod create_file;
mod dto;
mod get_file;

use actix_web::web::{self, Data};

use crate::{api::middlewares::authenticate::JwtAuth, config::Config};

pub(super) fn configure(config: Data<Config>) -> impl Fn(&mut web::ServiceConfig) {
    move |cfg| {
        cfg.service(
            web::resource("")
                .wrap(JwtAuth::new(config.clone()))
                .post(create_file::create_file),
        )
        .service(get_file::get_file);
    }
}
