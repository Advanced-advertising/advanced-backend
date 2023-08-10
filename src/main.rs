extern crate actix;
extern crate diesel;
extern crate diesel_migrations;

mod actors;
mod config;
mod db_utils;
mod errors;
mod files;
mod handlers;
mod middleware;
mod models;
mod schema;

use crate::config::Config;
use crate::middleware::token::validator;
use crate::models::app_state::AppState;
use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use dotenv::dotenv;
use slog::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let config = Config::from_env();
    let logger = Config::configure_log();

    info!(
        logger,
        "Starting server at https://{}:{}", config.server.host, config.server.port
    );

    HttpServer::new(move || {
        let bearer_middleware = HttpAuthentication::bearer(validator);
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();
        App::new()
            .app_data(Data::new(AppState {
                db: config.db.clone(),
                logger: logger.clone(),
            }))
            .wrap(cors)
            .wrap(actix_web::middleware::Logger::default())
            .service(
                web::scope("/categories")
                    .service(handlers::category::create)
                    .service(handlers::category::get_categories)
                    .service(handlers::category::update),
            )
            .service(
                web::scope("/users")
                    .service(handlers::user::register)
                    .service(handlers::user::login)
                    .service(
                        web::scope("")
                            .wrap(bearer_middleware.clone())
                            .service(handlers::user::change_img),
                    ),
            )
            .service(
                web::scope("/businesses")
                    .service(handlers::business::get_all)
                    .service(handlers::business::register)
                    .service(handlers::business::login)
                    .service(
                        web::scope("")
                            .wrap(bearer_middleware.clone())
                            .service(handlers::business::change_img),
                    ),
            )
            .service(web::scope("/screens").wrap(bearer_middleware))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
