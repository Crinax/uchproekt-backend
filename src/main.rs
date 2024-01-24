mod config;
mod api;
mod state;
mod cache;
mod db;
mod services;

use std::sync::Arc;

use dotenvy::dotenv;

use actix_web::{error, middleware::Logger, web, App, HttpServer, http::header};
use api::errors::invalid_data;
use cache::Cache;
use config::Config;
use env_logger::Env;
use actix_cors::Cors;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database};

use crate::{db::DbUrlProvider, services::{auth::AuthService, product::ProductService}};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let config = web::Data::new(Config::default());
    let host = config.host().to_owned();
    let port = config.port();
    let cache = Cache::new(config.redis_url()).expect("Redis instance error");
    let mut connection_options = ConnectOptions::new(config.db_url());

    connection_options.sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);

    let db = Database::connect(connection_options).await.expect("Db instance error");

    let cache_data = web::Data::new(cache);
    let product_service = web::Data::new(ProductService::new(db.clone()));
    let auth_service = web::Data::new(AuthService::new(db.clone()));

    log::info!("Running migrations...");
    Migrator::up(&db, None).await.expect("Error running migrations");
    log::info!("Migrations successfully applied!");

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
            .app_data(config.clone())
            .app_data(cache_data.clone())
            .app_data(product_service.clone())
            .app_data(auth_service.clone())
            .wrap(Logger::default())
            .service(web::scope("/api").configure(api::configure(config.clone())))
    })
    .bind((host, port))?
    .run()
    .await
}
