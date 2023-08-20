use crate::errors::AppErrorType::{AuthorizeError, IoError};
use actix::MailboxError;
use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use diesel::r2d2::{Error, PoolError};
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub enum AppErrorType {
    DbError,
    UnverifiedAdError,
    RejectedAdError,
    NotFoundError,
    SomethingWentWrong,
    PasswordOrLoginError,
    AuthorizeError,
    IoError,
}

#[derive(Debug)]
pub struct AppError {
    pub message: Option<String>,
    pub cause: Option<String>,
    pub error_type: AppErrorType,
}

impl AppError {
    pub fn new(message: Option<String>, cause: Option<String>, error_type: AppErrorType) -> Self {
        AppError {
            message,
            cause,
            error_type,
        }
    }
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

impl From<argonautica::Error> for AppError {
    fn from(error: argonautica::Error) -> Self {
        AppError {
            message: None,
            cause: Some(error.to_string()),
            error_type: AuthorizeError,
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError {
            message: None,
            cause: Some(error.to_string()),
            error_type: IoError,
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
            AppErrorType::DbError | AppErrorType::SomethingWentWrong => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            AppErrorType::NotFoundError => StatusCode::NOT_FOUND,
            AppErrorType::PasswordOrLoginError
            | AppErrorType::UnverifiedAdError
            | AppErrorType::RejectedAdError => StatusCode::BAD_REQUEST,
            AuthorizeError => StatusCode::INTERNAL_SERVER_ERROR,
            IoError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(AppErrorResponse {
            error: self.message(),
        })
    }
}
