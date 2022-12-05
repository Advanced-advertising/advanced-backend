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

use std::env;
use actix_web::{App, HttpServer, web};
use actix::SyncArbiter;
use actix_web::web::get;
use actix_web_httpauth::middleware::HttpAuthentication;
use crate::actors::db::DbActor;
use crate::db_utils::{get_pool};
use crate::handlers::user::{user_login, register_user, get_screens};
use crate::middleware::token::validator;
use crate::models::app_state::AppState;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_url = dotenv::var("DATABASE_URL").expect("Error retrieving the database url");
    // run_migrations(&db_url);
    let pool = get_pool(&db_url);
    let db_addr = SyncArbiter::start(8, move || DbActor(pool.clone()));

    HttpServer::new(move || {
        let bearer_middleware = HttpAuthentication::bearer(validator);
        App::new()
            .service(register_user)
            .service(user_login)
            .service(
                web::scope("/screens")
                    .wrap(bearer_middleware)
                    .service(get_screens)
            )
            .app_data(AppState {
                db: db_addr.clone()
            })
    })
    .bind(("0.0.0.0", 4000))?
    .run()
    .await
}
