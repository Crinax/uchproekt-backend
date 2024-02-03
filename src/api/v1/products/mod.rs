mod delete_products;
mod get_products;
// mod create_product;
mod dto;

use actix_web::web::{self, Data};

use crate::{api::middlewares::authenticate::JwtAuth, config::Config};

pub(super) fn configure(config: Data<Config>) -> impl Fn(&mut web::ServiceConfig) {
    move |cfg| {
        cfg
            .service(get_products::get_products)
            .service(get_products::get_concreate_product)
            .service(
                web::resource("")
                    .wrap(JwtAuth::new(config.clone()))
                    .delete(delete_products::delete_products)
            );
    }
}
