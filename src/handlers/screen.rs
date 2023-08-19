use crate::actors::address::GetAllAddresses;
use crate::actors::screens::{GetAllScreens, GetAllScreensByBusinessId, GetScreenDataById};
use crate::errors::AppError;
use crate::handlers::log_error;
use crate::models::app_state::AppState;
use actix_web::web::{Data, Json, ReqData};
use actix_web::{get, HttpResponse, post, Responder};
use slog::o;
use uuid::Uuid;
use crate::middleware::token::TokenClaims;

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

#[post("/get_all_by_business_id")]
pub async fn get_all_by_business_id(
    business_id: Json<Uuid>,
    state: Data<AppState>
) -> Result<impl Responder, AppError> {
    let db = state.as_ref().db.clone();
    let business_id = business_id.into_inner();
    let result = match db
        .send(GetAllScreensByBusinessId {
            business_id,
            logger: state.logger.clone(),
        })
        .await
    {
        Ok(res) => res,
        Err(err) => return Err(AppError::from_mailbox(err)),
    };

    let sub_log = state.logger.new(o!("handle" => "get_all_screens_by_business_id"));
    result
        .map(|screens| HttpResponse::Ok().json(screens))
        .map_err(log_error(sub_log))
}

#[get("/get_all_business_screens")]
pub async fn get_all_business_screens(
    req: Option<ReqData<TokenClaims>>,
    state: Data<AppState>
) -> Result<impl Responder, AppError> {
    match req {
        Some(business) => {
            let db = state.as_ref().db.clone();
            let result = match db
                .send(GetAllScreensByBusinessId {
                    business_id: business.id,
                    logger: state.logger.clone(),
                })
                .await
            {
                Ok(res) => res,
                Err(err) => return Err(AppError::from_mailbox(err)),
            };

            let sub_log = state.logger.new(o!("handle" => "get_all_business_screens"));
            result
                .map(|screens| HttpResponse::Ok().json(screens))
                .map_err(log_error(sub_log))
        }
        _ => Ok(HttpResponse::Unauthorized().json("Unable to verify identity")),
    }

}

#[post("/get_screen_data_by_id")]
pub async fn get_screen_data_by_id(
    screen_id: Json<Uuid>,
    state: Data<AppState>
) -> Result<impl Responder, AppError> {
    let db = state.as_ref().db.clone();
    let screen_id = screen_id.into_inner();
    let result = match db
        .send(GetScreenDataById {
            screen_id,
            logger: state.logger.clone(),
        })
        .await
    {
        Ok(res) => res,
        Err(err) => return Err(AppError::from_mailbox(err)),
    };

    let sub_log = state.logger.new(o!("handle" => "get_screen_data_by_id"));
    result
        .map(|screen_data| HttpResponse::Ok().json(screen_data))
        .map_err(log_error(sub_log))
}

#[get("/get_all_addresses")]
pub async fn get_all_addresses(state: Data<AppState>) -> Result<impl Responder, AppError> {
    let db = state.as_ref().db.clone();
    let result = match db
        .send(GetAllAddresses {
            logger: state.logger.clone(),
        })
        .await
    {
        Ok(res) => res,
        Err(err) => return Err(AppError::from_mailbox(err)),
    };

    let sub_log = state.logger.new(o!("handle" => "get_all_addresses"));
    result
        .map(|addresses| HttpResponse::Ok().json(addresses))
        .map_err(log_error(sub_log))
}