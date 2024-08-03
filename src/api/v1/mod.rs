mod auth;
mod categories;
mod company_services;
mod fields;
mod files;
mod orders;
mod products;

pub use products::FieldInProductDto;

use actix_web::web::{self, Data};

use crate::config::Config;

pub(super) fn configure(config: Data<Config>) -> impl Fn(&mut web::ServiceConfig) {
    move |cfg| {
        cfg.service(web::scope("/products").configure(products::configure(config.clone())))
            .service(web::scope("/auth").configure(auth::configure()))
            .service(web::scope("/categories").configure(categories::configure(config.clone())))
            .service(web::scope("/files").configure(files::configure(config.clone())))
            .service(web::scope("/orders").configure(orders::configure(config.clone())))
            .service(web::scope("/fields").configure(fields::configure(config.clone())))
            .service(
                web::scope("/services").configure(company_services::configure(config.clone())),
            );
    }
}
