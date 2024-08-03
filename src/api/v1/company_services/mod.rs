mod delete_company_services;
mod dto;
mod get_company_services;
mod update_company_services;

use actix_web::web::{self, Data};

use crate::{api::middlewares::authenticate::JwtAuth, config::Config};

pub(super) fn configure(config: Data<Config>) -> impl Fn(&mut web::ServiceConfig) {
    move |cfg| {
        cfg.service(get_company_services::get_company_services)
        .service(
            web::resource("{id}")
                .wrap(JwtAuth::new(config.clone()))
                .patch(update_company_services::update_service)
                .delete(delete_company_services::delete_service),
        );
    }
}
