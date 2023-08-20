use actix_web::{get, HttpResponse, Responder};
use actix_web::web::{Data, ReqData};
use slog::o;
use crate::actors::income::GetAllIncomes;
use crate::errors::AppError;
use crate::handlers::log_error;
use crate::middleware::token::TokenClaims;
use crate::models::app_state::AppState;

#[get("/get_all_business_incomes")]
pub async fn get_all_business_screens(
    req: Option<ReqData<TokenClaims>>,
    state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    match req {
        Some(business) => {
            let db = state.as_ref().db.clone();
            let result = match db
                .send(GetAllIncomes{
                    business_id: business.id,
                    logger: state.logger.clone(),
                })
                .await
            {
                Ok(res) => res,
                Err(err) => return Err(AppError::from_mailbox(err)),
            };

            let sub_log = state.logger.new(o!("handle" => "get_all_business_incomes"));
            result
                .map(|screens| HttpResponse::Ok().json(screens))
                .map_err(log_error(sub_log))
        }
        _ => Ok(HttpResponse::Unauthorized().json("Unable to verify identity")),
    }
}