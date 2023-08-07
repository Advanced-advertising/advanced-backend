use actix::MailboxError;
use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use diesel::r2d2::{Error, PoolError};
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub enum AppErrorType {
    DbError,
    NotFoundError,
    ConfigError,
    SomethingWentWrong,
    PasswordOrLoginError,
}

#[derive(Debug)]
pub struct AppError {
    pub message: Option<String>,
    pub cause: Option<String>,
    pub error_type: AppErrorType,
}

impl AppError {
    pub fn from_mailbox(error: MailboxError) -> Self {
        Self {
            message: Some("Something went wrong".to_string()),
            cause: Some(error.to_string()),
            error_type: AppErrorType::SomethingWentWrong,
        }
    }

    pub fn message(&self) -> String {
        match &*self {
            AppError {
                message: Some(message),
                ..
            } => message.clone(),
            AppError {
                message: None,
                error_type: AppErrorType::NotFoundError,
                ..
            } => "The requested item was not found".to_string(),
            _ => "An unexpected error has occurred".to_string(),
        }
    }
}

impl From<PoolError> for AppError {
    fn from(error: PoolError) -> AppError {
        AppError {
            message: None,
            cause: Some(error.to_string()),
            error_type: AppErrorType::DbError,
        }
    }
}

impl From<Error> for AppError {
    fn from(error: Error) -> AppError {
        AppError {
            message: None,
            cause: Some(error.to_string()),
            error_type: AppErrorType::DbError,
        }
    }
}

impl From<diesel::result::Error> for AppError {
    fn from(error: diesel::result::Error) -> AppError {
        AppError {
            message: None,
            cause: Some(error.to_string()),
            error_type: AppErrorType::DbError,
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize)]
pub struct AppErrorResponse {
    pub error: String,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self.error_type {
            AppErrorType::DbError
            | AppErrorType::ConfigError
            | AppErrorType::SomethingWentWrong => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::NotFoundError => StatusCode::NOT_FOUND,
            AppErrorType::PasswordOrLoginError => StatusCode::BAD_REQUEST,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(AppErrorResponse {
            error: self.message(),
        })
    }
}
