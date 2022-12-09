use slog::{error, Logger, o};
use crate::errors::AppError;

pub mod user;
pub mod business;

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
