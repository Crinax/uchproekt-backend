mod products;

use actix_web::web;

pub(super) fn configure() -> impl Fn(&mut web::ServiceConfig) {
    move |cfg| {
        cfg.service(
            web::scope("/products").configure(products::configure())
        );
    }
}
