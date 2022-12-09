use crate::actors::user::{AuthorizeUser, CreateUser};
use crate::errors::{AppError, AppErrorType};
use crate::files::save_files;
use crate::handlers::log_error;
use crate::middleware::token::TokenClaims;
use crate::models::app_state::AppState;
use crate::models::user::{User, UserData};
use actix_form_data::{Form, Value};
use actix_multipart;
use actix_multipart::Multipart;
use actix_web::web::{Data, Json, ReqData};
use actix_web::{get, post, web, HttpResponse, Responder};
use actix_web_httpauth::extractors::basic::BasicAuth;
use futures_util::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use slog::{error, info, log, o, Logger};
use std::fmt::Binary;
use std::fs::File;
use std::io::Write;

#[post("/register")]
pub async fn register(
    user: Json<UserData>,
    state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    let db = state.as_ref().db.clone();
    let user = user.into_inner();

    let result = match db
        .send(CreateUser {
            name: user.user_name,
            email: user.email,
            password: user.password,
            logger: state.logger.clone(),
        })
        .await
    {
        Ok(res) => res,
        Err(err) => return Err(AppError::from_mailbox(err)),
    };

    let sub_log = state.logger.new(o!("handle" => "create_user"));
    result
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(log_error(sub_log))
}

#[get("/login")]
pub async fn login(
    basic_auth: BasicAuth,
    state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    let db = state.as_ref().db.clone();
    let password = match basic_auth.password() {
        Some(pass) => pass,
        None => {
            return Err(AppError {
                message: Some("Must provide username and password".to_string()),
                cause: None,
                error_type: AppErrorType::SomethingWentWrong,
            })
        }
    };

    let authorise_user = AuthorizeUser {
        name: basic_auth.user_id().to_string(),
        password: password.into(),
    };

    let result = match db.send(authorise_user).await {
        Ok(res) => res,
        Err(err) => return Err(AppError::from_mailbox(err)),
    };

    let sub_log = state.logger.new(o!("handle" => "login"));

    result
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(log_error(sub_log))
}
