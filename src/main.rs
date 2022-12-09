extern crate actix;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod schema;
mod actors;
mod handlers;
mod models;
mod db_utils;
mod middleware;
mod errors;
mod config;

use std::env;
use actix_web::{App, http, HttpServer, web};
use actix_cors::Cors;
use actix::SyncArbiter;
use actix_form_data::{Error, Field, Form};
use actix_web::web::{Data, get};
use actix_web_httpauth::middleware::HttpAuthentication;
use dotenv::dotenv;
use slog::{debug, info};
use crate::actors::db::DbActor;
use crate::config::Config;
use crate::db_utils::{get_pool};
use crate::handlers::user::{user_login, register_user, get_screens};
use crate::middleware::token::validator;
use crate::models::app_state::AppState;
use futures_util::stream::StreamExt;

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
            .service(register_user)
            .service(user_login)
            .service(
                web::scope("/screens")
                    .wrap(bearer_middleware)
                    .service(get_screens)
            )
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
