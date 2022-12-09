use actix::Message;
use actix_web::{HttpResponse, Responder};
use actix_web::web::Data;
use serde::{Deserialize, Serialize};
use crate::errors::AppError;
use slog::{error, o, Logger};
use crate::actors::db::DbActor;
use crate::models::app_state::AppState;

pub mod business;
pub mod user;

fn log_error(log: Logger) -> impl Fn(AppError) -> AppError {
    move |err| {
        let log = log.new(o!(
            "cause" => err.cause.clone()
        ));
        match err.message.clone() {
            Some(message) => error!(log, "{}", message),
            None => error!(log, "Something went wrong"),
        }

        AppError::from(err)
    }
}
