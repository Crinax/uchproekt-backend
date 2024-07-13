mod create_product;
mod delete_products;
mod dto;
mod get_products;

use actix_web::web::{self, resource, Data};

use crate::{api::middlewares::authenticate::JwtAuth, config::Config};

pub(super) fn configure(config: Data<Config>) -> impl Fn(&mut web::ServiceConfig) {
    move |cfg| {
        cfg.service(get_products::get_products)
            .service(get_products::get_concreate_product)
            .service(
                web::resource("")
                    .wrap(JwtAuth::new(config.clone()))
                    .post(create_product::create_product),
            )
            .service(
                web::resource("{id}")
                    .wrap(JwtAuth::new(config.clone()))
                    .delete(delete_products::delete_products),
            );
    }
}
