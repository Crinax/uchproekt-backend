mod config;
mod api;
mod state;
mod cache;
mod db;

use std::sync::Arc;

use dotenvy::dotenv;

use actix_web::{error, middleware::Logger, web, App, HttpServer, http::header};
use api::errors::invalid_data;
use cache::Cache;
use config::Config;
use state::AppState;
use env_logger::Env;
use actix_cors::Cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let config = Arc::new(Config::default());
    let cache = Cache::new(config.redis_url()).expect("Redis instance error");

    let data = web::Data::new(AppState::new(
        config.clone(),
        cache,
    ));

    let json_cfg = web::JsonConfig::default()
        .limit(4096)
        .error_handler(|err, _req| {
            log::error!("{:?}", err);
            error::InternalError::from_response(err, invalid_data()).into()
        });

    log::info!("Starting server at {}:{}", config.host(), config.port());

    HttpServer::new(move || {
        let cors = Cors::default()
              .allow_any_origin()
              .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
              .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::CONTENT_TYPE,
                header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
                header::CONTENT_TYPE
              ])
              .supports_credentials()
              .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(json_cfg.clone())
            .app_data(data.clone())
            .wrap(Logger::default())
    })
    .bind((config.host(), config.port()))?
    .run()
    .await
}
