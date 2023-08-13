use crate::actors::ad::{CreateAd, GetAllAds, UpdateAd};
use crate::errors::AppError;
use crate::handlers::log_error;
use crate::middleware::token::TokenClaims;
use crate::models::ad::{AdData, AdDataUpdate};
use crate::models::app_state::AppState;
use actix_web::web::{Data, Json, ReqData};
use actix_web::{get, post, HttpResponse, Responder};
use slog::o;

#[post("/create")]
pub async fn create(
    ad_data: Json<AdData>,
    req: Option<ReqData<TokenClaims>>,
    state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    match req {
        Some(user) => {
            let db = state.as_ref().db.clone();
            let ad_data = ad_data.into_inner();

            let result = match db
                .send(CreateAd {
                    ad_name: ad_data.ad_name,
                    user_id: user.id,
                    img_url: ad_data.img_url,
                    logger: state.logger.clone(),
                })
                .await
            {
                Ok(res) => res,
                Err(err) => return Err(AppError::from_mailbox(err)),
            };

            let sub_log = state.logger.new(o!("handle" => "create_category"));
            result
                .map(|ad| HttpResponse::Ok().json(ad))
                .map_err(log_error(sub_log))
        }
        _ => Ok(HttpResponse::Unauthorized().json("Unable to verify identity")),
    }
}

#[post("/update")]
pub async fn update(
    ad: Json<AdDataUpdate>,
    state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    let db = state.as_ref().db.clone();
    let ad = ad.into_inner();

    let result = match db
        .send(UpdateAd {
            id: ad.ad_id,
            ad_name: ad.ad_name,
            img_url: ad.img_url,
            user_id: ad.user_id,
            logger: state.logger.clone(),
        })
        .await
    {
        Ok(res) => res,
        Err(err) => return Err(AppError::from_mailbox(err)),
    };

    let sub_log = state.logger.new(o!("handle" => "update_category"));
    result
        .map(|category| HttpResponse::Ok().json(category))
        .map_err(log_error(sub_log))
}

#[get("/get_all")]
pub async fn get_ads(state: Data<AppState>) -> Result<impl Responder, AppError> {
    let db = state.as_ref().db.clone();
    let result = match db.send(GetAllAds).await {
        Ok(res) => res,
        Err(err) => return Err(AppError::from_mailbox(err)),
    };

    let sub_log = state.logger.new(o!("handle" => "create_category"));
    result
        .map(|categories| HttpResponse::Ok().json(categories))
        .map_err(log_error(sub_log))
}
