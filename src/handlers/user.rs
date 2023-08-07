use crate::actors::user::{AuthorizeUser, CreateUser};
use crate::errors::{AppError, AppErrorType};
use crate::handlers::log_error;
use crate::models::app_state::AppState;
use crate::models::user::UserData;
use actix_web::web::{Data, Json};
use actix_web::{get, post, HttpResponse, Responder};
use actix_web_httpauth::extractors::basic::BasicAuth;
use futures_util::{StreamExt};
use slog::o;
use crate::middleware::token::get_password;

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
    let password = get_password(basic_auth.clone())?;

    let authorise_user = AuthorizeUser {
        name: basic_auth.user_id().to_string(),
        password,
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
