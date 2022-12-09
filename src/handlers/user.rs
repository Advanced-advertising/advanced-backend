use std::fmt::Binary;
use std::fs::File;
use std::io::Write;
use actix_web::{HttpResponse, Responder, post, get, web};
use actix_web::web::{Data, Json, ReqData};
use actix_form_data::{Form, Value};
use actix_web_httpauth::extractors::basic::BasicAuth;
use actix_multipart;
use actix_multipart::Multipart;
use futures_util::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use slog::{error, info, log, Logger, o};
use crate::actors::user::{AuthorizeUser, CreateUser};
use crate::errors::{AppError, AppErrorType};
use crate::handlers::log_error;
use crate::middleware::token::TokenClaims;
use crate::models::app_state::AppState;
use crate::models::user::{User, UserData};

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

#[get("/user_login")]
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

#[derive(Deserialize, Serialize)]
pub struct FormData {
    name: String,
}

#[post("/get_screens")]
pub async fn get_screens(
    mut payload: Multipart,
    req: Option<ReqData<TokenClaims>>,
    state: Data<AppState>
) -> impl Responder {

    match req {
        Some(user) => {
            let upload_status = save_file(payload, "filename.png".to_string()).await;
            match upload_status {
                Ok(_) => {
                    HttpResponse::Ok()
                        .content_type("text/plain")
                        .body("update_succeeded")
                }
                _ => HttpResponse::BadRequest()
                    .content_type("text/plain")
                    .body("update_failed"),
            }
        },
        _ => HttpResponse::Unauthorized().json("Unable to verify identity"),
    }
}

pub async fn save_file(mut payload: Multipart, file_path: String) -> Result<bool, std::io::Error> {
    while let Ok(Some(mut field)) = payload.try_next().await {
        let filepath = format!(".{}", file_path);

        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap()?;

        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f = web::block(move || f.write_all(&data).map(|_| f))
                .await
                .unwrap()?;
        }
    }

    Ok(true)
}