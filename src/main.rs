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

use std::env;
use actix_web::{App, HttpServer};
use actix::SyncArbiter;
use crate::actors::db::DbActor;
use crate::db_utils::{get_pool};
use crate::handlers::user::register_user;
use crate::models::app_state::AppState;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_url = env::var("DATABASE_URL").expect("Error retrieving the database url");
    // run_migrations(&db_url);
    let pool = get_pool(&db_url);
    let db_addr = SyncArbiter::start(8, move || DbActor(pool.clone()));

    HttpServer::new(move || {
        App::new()
            .service(register_user)
            .app_data(AppState {
                db: db_addr.clone()
            })
    })
    .bind(("0.0.0.0", 4000))?
    .run()
    .await
}
