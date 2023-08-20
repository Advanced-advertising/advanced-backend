use crate::actors::ad_order::{CreateAdOrder, GetBusinessAdOrders};
use crate::errors::AppError;
use crate::handlers::log_error;
use crate::middleware::token::TokenClaims;
use crate::models::app_state::AppState;
use actix_web::web::{Data, Json, ReqData};
use actix_web::{get, HttpResponse, post, Responder};
use slog::o;
use crate::models::ad_order::AdOrderData;

#[get("/get_business_ad_orders")]
pub async fn get_business_ad_orders(
    req: Option<ReqData<TokenClaims>>,
    state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    match req {
        Some(business) => {
            let db = state.as_ref().db.clone();
            let business_id = business.id;
            let result = match db
                .send(GetBusinessAdOrders {
                    business_id,
                    logger: state.logger.clone(),
                })
                .await
            {
                Ok(res) => res,
                Err(err) => return Err(AppError::from_mailbox(err)),
            };

            let sub_log = state.logger.new(o!("handle" => "create_category"));
            result
                .map(|ad_orders| HttpResponse::Ok().json(ad_orders))
                .map_err(log_error(sub_log))
        }
        _ => Ok(HttpResponse::Unauthorized().json("Unable to verify identity")),
    }
}

#[post("/create_ad_order")]
pub async fn create_ad_order(
    ad_order_data: Json<AdOrderData>,
    state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    let db = state.as_ref().db.clone();
    let ad_order_data = ad_order_data.into_inner();

    let result = match db
        .send(CreateAdOrder {
            start_time: ad_order_data.start_time,
            end_time: ad_order_data.end_time,
            price: ad_order_data.price,
            ad_id: ad_order_data.ad_id,
            screen_id: ad_order_data.screen_id,
            logger: state.logger.clone(),
        })
        .await
    {
        Ok(res) => res,
        Err(err) => return Err(AppError::from_mailbox(err)),
    };

    let sub_log = state.logger.new(o!("handle" => "create_ad_order"));
    result
        .map(|ad_order| HttpResponse::Ok().json(ad_order))
        .map_err(log_error(sub_log))
}
