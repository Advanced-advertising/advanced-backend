use std::os::linux::raw::stat;
use actix_web::{HttpResponse, Responder, post, get};
use actix_web::web::{Data, Json, ReqData};
use actix_web_httpauth::extractors::basic::BasicAuth;
use crate::actors::user::{AuthorizeUser, CreateUser};
use crate::middleware::token::TokenClaims;
use crate::models::app_state::AppState;
use crate::models::user::{User, UserData};

#[post("/register")]
pub async fn register_user(user: Json<UserData>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let user = user.into_inner();

    match db.send(CreateUser {
        name: user.user_name,
        email: user.email,
        password: user.password,
    }).await {
        Ok(Ok(user)) => HttpResponse::Ok().json(user),
        _ => HttpResponse::InternalServerError().json("Something went wrong")
    }
}

#[post("/user_login")]
pub async fn user_login(basic_auth: BasicAuth, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let password = match basic_auth.password() {
        Some(pass) => pass,
        None => return HttpResponse::BadRequest().json("Must provide username and password"),
    };

    let authorise_user = AuthorizeUser {
        name: basic_auth.user_id().into(),
        password: password.into(),
    };

    match db.send(authorise_user).await {
        Ok(Ok(token)) => HttpResponse::Ok().json(token),
        _ => HttpResponse::InternalServerError().json("Something went wrong")
    }
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