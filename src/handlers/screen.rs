use actix_web::{get, HttpResponse, Responder};
use actix_web::web::Data;
use slog::o;
use crate::errors::AppError;
use crate::handlers::log_error;
use crate::models::app_state::AppState;
use crate::actors::screens::GetAllScreens;

#[get("/get_all")]
pub async fn get_all(state: Data<AppState>) -> Result<impl Responder, AppError> {
    let db = state.as_ref().db.clone();
    let result = match db
        .send(GetAllScreens {
            logger: state.logger.clone(),
        })
        .await
    {
        Ok(res) => res,
        Err(err) => return Err(AppError::from_mailbox(err)),
    };

    let sub_log = state.logger.new(o!("handle" => "get_all_screens"));
    result
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(log_error(sub_log))
}
