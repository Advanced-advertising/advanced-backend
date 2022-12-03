use actix_web::{HttpResponse, Responder, post};
use actix_web::web::{Data, Json};
use crate::actors::user::CreateUser;
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