mod delete_products;
mod get_products;
mod dto;

use actix_web::web::{self, Data};

use crate::{api::middlewares::authenticate::JwtAuth, config::Config};

pub(super) fn configure(config: Data<Config>) -> impl Fn(&mut web::ServiceConfig) {
    move |cfg| {
        cfg.service(get_products::get_products).service(
            web::scope("")
                .service(delete_products::delete_products)
                .wrap(JwtAuth::new(config.clone())),
        );
    }
}
