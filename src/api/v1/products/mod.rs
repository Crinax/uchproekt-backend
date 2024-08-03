mod create_product;
mod delete_products;
mod dto;
mod get_products;
mod update_product;

pub use dto::FieldInProductDto;

use actix_web::web::{self, Data};

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
                    .delete(delete_products::delete_products)
                    .patch(update_product::update_product),
            )
            .service(
                web::resource("{product_id}/fields/{field_id}")
                    .wrap(JwtAuth::new(config.clone()))
                    .patch(update_product::add_or_update_field_to_product)
                    .delete(delete_products::delete_field_from_product),
            );
    }
}
