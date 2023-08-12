use crate::errors::AppError;
use slog::{error, o, Logger};
use std::io;

pub mod ad;
pub mod ad_order;
pub mod admin;
pub mod business;
pub mod category;
pub mod images;
pub mod payment;
pub mod screen;
pub mod user;

fn log_io_error(log: Logger) -> impl Fn(io::Error) -> AppError {
    move |err| {
        let app_error: AppError = err.into();

        let log = log.new(o!(
            "cause" => app_error.cause.clone()
        ));
        match app_error.message.clone() {
            Some(message) => error!(log, "{}", message),
            None => error!(log, "Something went wrong"),
        }

        AppError::from(app_error)
    }
}

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
