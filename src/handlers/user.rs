use std::os::linux::raw::stat;
use actix_web::{HttpResponse, Responder, post, get};
use actix_web::web::{Data, Json, ReqData};
use actix_web_httpauth::extractors::basic::BasicAuth;
use slog::{error, Logger, o};
use crate::actors::user::{AuthorizeUser, CreateUser};
use crate::errors::{AppError, AppErrorType};
use crate::middleware::token::TokenClaims;
use crate::models::app_state::AppState;
use crate::models::user::{User, UserData};


fn log_error(log: Logger) -> impl Fn(AppError) -> AppError {
    move |err| {
        let log = log.new(o!(
            "cause" => err.cause.clone()
        ));
        let message = err.message.clone().unwrap();
        error!(log, "{}", message);
        AppError::from(err)
    }
}

#[post("/register")]
pub async fn register_user(user: Json<UserData>, state: Data<AppState>) -> Result<impl Responder, AppError> {
    let db = state.as_ref().db.clone();
    let user = user.into_inner();

    let result = match db.send(CreateUser {
        name: user.user_name,
        email: user.email,
        password: user.password,
        logger: state.logger.clone(),
    }).await {
        Ok(res) => res,
        Err(err) => return Err(AppError::from_mailbox(err))
    };

    let sub_log = state.logger.new(o!("handle" => "create_user"));
    result.map(|user| HttpResponse::Ok().json(user)).map_err(log_error(sub_log))
}

#[post("/user_login")]
pub async fn user_login(basic_auth: BasicAuth, state: Data<AppState>) -> Result<impl Responder, AppError> {
    let db = state.as_ref().db.clone();
    let password = match basic_auth.password() {
        Some(pass) => pass,
        None => return Err(AppError {
            message: Some("Must provide username and password".to_string()),
            cause: None,
            error_type: AppErrorType::SomethingWentWrong,
        }),
    };

    let authorise_user = AuthorizeUser {
        name: basic_auth.user_id().to_string(),
        password: password.into(),
    };

    let result = match db.send(authorise_user).await {
        Ok(res) => res,
        Err(err) => return Err(AppError::from_mailbox(err))
    };

    let sub_log = state.logger.new(o!("handle" => "login_user"));
    result.map(|user| HttpResponse::Ok().json(user)).map_err(log_error(sub_log))
}

#[get("/get_screens")]
pub async fn get_screens(
    body: Json<String>,
    req: Option<ReqData<TokenClaims>>,
    state: Data<AppState>
) -> impl Responder {
    match req {
        Some(user) => {
            HttpResponse::Ok().json(body)
        },
        _ => HttpResponse::Unauthorized().json("Unable to verify identity"),
    }
}