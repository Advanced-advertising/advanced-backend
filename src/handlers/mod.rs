use crate::actors::db::DbActor;
use crate::errors::AppError;
use crate::models::app_state::AppState;
use actix::Message;
use actix_web::web::Data;
use actix_web::{HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use slog::{error, o, Logger};

pub mod business;
pub mod category;
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
