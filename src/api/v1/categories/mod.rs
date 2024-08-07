mod create_category;
mod delete_category;
mod dto;
mod get_categories;
mod patch_category;

use actix_web::web::{self, Data};

use crate::{api::middlewares::authenticate::JwtAuth, config::Config};

pub(super) fn configure(config: Data<Config>) -> impl Fn(&mut web::ServiceConfig) {
    move |cfg| {
        cfg.service(get_categories::get_categories)
            .service(get_categories::get_tree_categories)
            .service(get_categories::get_category_with_products)
            .service(
                web::resource("")
                    .wrap(JwtAuth::new(config.clone()))
                    .post(create_category::create_category),
            )
            .service(
                web::resource("{id}")
                    .wrap(JwtAuth::new(config.clone()))
                    .patch(patch_category::patch_category)
                    .delete(delete_category::delete_categories),
            );
    }
}
