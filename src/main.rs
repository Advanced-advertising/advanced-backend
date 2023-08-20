extern crate actix;
extern crate diesel;
extern crate diesel_migrations;

mod actors;
mod config;
mod db_utils;
mod errors;
mod handlers;
mod middleware;
mod models;
mod schema;

use crate::config::Config;
use crate::middleware::token::validator;
use crate::middleware::token::Role::{Admin, Business as BusinessRole, Client};
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
        "Starting server at http://{}:{}", config.server.host, config.server.port
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
            .service(web::scope("/images").service(handlers::images::get_image))
            .service(
                web::scope("/categories")
                    .wrap(bearer_middleware.clone())
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
                            .app_data(Data::new(vec![Client, Admin]))
                            .service(handlers::user::change_img)
                            .service(handlers::ad_order::create_ad_order),
                    ),
            )
            .service(
                web::scope("/ad")
                    .wrap(bearer_middleware.clone())
                    .service(handlers::ad::create)
                    .service(handlers::ad::get_ads)
                    .service(handlers::ad::update),
            )
            .service(
                web::scope("/screens")
                    .service(handlers::screen::find_optimal_screens)
                    .service(
                        web::scope("")
                            .wrap(bearer_middleware.clone())
                            .service(handlers::screen::get_all)
                            .service(handlers::screen::get_screen_data_by_id)
                            .service(handlers::screen::get_all_business_screens)
                            .service(handlers::screen::get_all_by_business_id)
                            .service(handlers::screen::get_all_addresses)
                    ),
            )
            .service(
                web::scope("/businesses")
                    .service(handlers::business::register)
                    .service(handlers::business::login)
                    .service(handlers::business::get_all)
                    .service(handlers::business::get_categories)
                    .service(handlers::business::get_business_info_by_id)
                    .service(
                        web::scope("")
                            .wrap(bearer_middleware.clone())
                            .app_data(Data::new(vec![BusinessRole, Admin]))
                            .service(handlers::business::get_business_info)
                            .service(handlers::ad_order::get_business_ad_orders)
                            .service(handlers::business::get_categories)
                            .service(handlers::income::get_all_business_screens)
                            .service(handlers::business::change_img)
                            .service(handlers::business::change_business_info)
                            .service(handlers::ad_order::reject_ad_order)
                            .service(handlers::ad_order::approve_ad_order)
                    ),
            )
            .service(
                web::scope("/admin")
                    .service(handlers::admin::register)
                    .service(handlers::admin::login)
                    .service(
                        web::scope("")
                            .wrap(bearer_middleware)
                            .app_data(Data::new(vec![Admin]))
                            .service(handlers::admin::create_screen)
                            .service(handlers::admin::create_address)
                            .service(handlers::admin::change_ad_status),
                    ),
            )
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
