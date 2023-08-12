use crate::errors::AppError;
use crate::handlers::log_error;
use crate::models::app_state::AppState;
use crate::models::user::UserData;
use actix_web::web::{Data, Json};
use actix_web::{get, post, HttpResponse, Responder};
use actix_web_httpauth::extractors::basic::BasicAuth;
use slog::o;
use crate::actors::admin::{AuthorizeAdmin, CreateAdmin};

#[post("/create")]
pub async fn register(
    user: Json<UserData>,
    state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    let db = state.as_ref().db.clone();
    let user = user.into_inner();

    let result = match db
        .send(CreateAdmin {
            name: user.user_name,
            password: user.password,
            logger: state.logger.clone(),
        })
        .await
    {
        Ok(res) => res,
        Err(err) => return Err(AppError::from_mailbox(err)),
    };

    let sub_log = state.logger.new(o!("handle" => "create_admin"));
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

    let authorise_user = AuthorizeAdmin { basic_auth };

    let result = match db.send(authorise_user).await {
        Ok(res) => res,
        Err(err) => return Err(AppError::from_mailbox(err)),
    };

    let sub_log = state.logger.new(o!("handle" => "login admin"));

    result
        .map(|token_str| HttpResponse::Ok().json(token_str))
        .map_err(log_error(sub_log))
}