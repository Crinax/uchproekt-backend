mod get_products;

use actix_web::web;

pub(super) fn configure() -> impl Fn(&mut web::ServiceConfig) {
    move |cfg| {
        cfg.service(get_products::get_products);
    }
}
