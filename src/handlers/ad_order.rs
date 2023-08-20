use actix_web::{get, HttpResponse, Responder};
use actix_web::web::{Data, ReqData};
use slog::o;
use crate::actors::ad_order::GetBusinessAdOrders;
use crate::errors::AppError;
use crate::handlers::log_error;
use crate::middleware::token::TokenClaims;
use crate::models::app_state::AppState;

#[get("/get_business_ad_orders")]
pub async fn get_business_ad_orders(
    req: Option<ReqData<TokenClaims>>,
    state: Data<AppState>
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
