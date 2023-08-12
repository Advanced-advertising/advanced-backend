use crate::actors::user::ChangeImg;
use crate::actors::user::{AuthorizeUser, CreateUser};
use crate::errors::AppError;
use crate::handlers::images::save_files;
use crate::handlers::log_error;
use crate::middleware::token::TokenClaims;
use crate::models::app_state::AppState;
use crate::models::user::UserData;
use actix_multipart::Multipart;
use actix_web::web::{Data, Json, ReqData};
use actix_web::{get, post, HttpResponse, Responder};
use actix_web_httpauth::extractors::basic::BasicAuth;
use slog::o;

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
            img_url: "".to_string(),
            phone_number: user.phone_number,
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

    let authorise_user = AuthorizeUser { basic_auth };

    let result = match db.send(authorise_user).await {
        Ok(res) => res,
        Err(err) => return Err(AppError::from_mailbox(err)),
    };

    let sub_log = state.logger.new(o!("handle" => "login client"));

    result
        .map(|token_str| HttpResponse::Ok().json(token_str))
        .map_err(log_error(sub_log))
}

#[post("/change_img")]
pub async fn change_img(
    payload: Multipart,
    req: Option<ReqData<TokenClaims>>,
    state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    match req {
        Some(user) => {
            let img_url = save_files(payload).await?;

            let change_img = ChangeImg {
                user_id: user.id,
                img_url,
            };

            let db = state.as_ref().db.clone();
            let result = match db.send(change_img).await {
                Ok(res) => res,
                Err(err) => return Err(AppError::from_mailbox(err)),
            };

            let sub_log = state.logger.new(o!("handle" => "change img for business"));

            result
                .map(|img_url| HttpResponse::Ok().json(img_url))
                .map_err(log_error(sub_log))
        }
        _ => Ok(HttpResponse::Unauthorized().json("Unable to verify identity")),
    }
}
