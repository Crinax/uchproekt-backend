mod create_order;
mod dto;
mod get_orders;

use actix_web::web::{self, Data};

use crate::{api::middlewares::authenticate::JwtAuth, config::Config};

pub(super) fn configure(config: Data<Config>) -> impl Fn(&mut web::ServiceConfig) {
    move |cfg| {
        cfg.service(
            web::resource("")
                .wrap(JwtAuth::new(config.clone()))
                .get(get_orders::get_orders)
                .post(create_order::create_order),
        );
    }
}
