use crate::actors::business::{AuthorizeBusiness, ChangeImg, CreateBusiness, GetAllBusinesses};
use crate::errors::AppError;
use crate::handlers::images::save_files;
use crate::handlers::log_error;
use crate::middleware::token::TokenClaims;
use crate::models::app_state::AppState;
use crate::models::business::BusinessData;
use actix_multipart::Multipart;
use actix_web::web::{Data, Json, ReqData};
use actix_web::{get, post, HttpResponse, Responder};
use actix_web_httpauth::extractors::basic::BasicAuth;
use slog::{o};
use crate::actors::address::CreateAddress;
use crate::models::address::AddressData;

#[get("/get_all")]
pub async fn get_all(state: Data<AppState>) -> Result<impl Responder, AppError> {
    let db = state.as_ref().db.clone();
    let result = match db
        .send(GetAllBusinesses {
            logger: state.logger.clone(),
        })
        .await
    {
        Ok(res) => res,
        Err(err) => return Err(AppError::from_mailbox(err)),
    };

    let sub_log = state.logger.new(o!("handle" => "get_all_businesses"));
    result
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(log_error(sub_log))
}

#[post("/register")]
pub async fn register(
    business: Json<BusinessData>,
    state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    let db = state.as_ref().db.clone();
    let business = business.into_inner();

    let result = match db
        .send(CreateBusiness {
            name: business.business_name,
            email: business.email,
            password: business.password,
            phone_number: business.phone_number,
            logger: state.logger.clone(),
            img_url: "".to_string(),
        })
        .await
    {
        Ok(res) => res,
        Err(err) => return Err(AppError::from_mailbox(err)),
    };

    let sub_log = state.logger.new(o!("handle" => "create_business"));
    result
        .map(|business| HttpResponse::Ok().json(business))
        .map_err(log_error(sub_log))
}

#[get("/login")]
pub async fn login(
    basic_auth: BasicAuth,
    state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    let authorise_business = AuthorizeBusiness { basic_auth };

    let db = state.as_ref().db.clone();
    let result = match db.send(authorise_business).await {
        Ok(res) => res,
        Err(err) => return Err(AppError::from_mailbox(err)),
    };

    let sub_log = state.logger.new(o!("handle" => "login business"));

    result
        .map(|business| HttpResponse::Ok().json(business))
        .map_err(log_error(sub_log))
}

#[post("/change_img")]
pub async fn change_img(
    payload: Multipart,
    req: Option<ReqData<TokenClaims>>,
    state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    match req {
        Some(business) => {
            let img_url = save_files(payload).await?;

            let change_img = ChangeImg {
                business_id: business.id,
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

#[post("/add_address")]
pub async fn add_address(
    address_data: Json<AddressData>,
    req: Option<ReqData<TokenClaims>>,
    state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    match req {
        Some(business) => {
            let db = state.as_ref().db.clone();
            let address_data = address_data.into_inner();

            let business_id = business.id;
            let result = match db
                .send(CreateAddress {
                    name: address_data.address_name,
                    business_id,
                    logger: state.logger.clone(),
                })
                .await
            {
                Ok(res) => res,
                Err(err) => return Err(AppError::from_mailbox(err)),
            };

            let sub_log = state.logger.new(o!("handle" => "add_address"));
            result
                .map(|business| HttpResponse::Ok().json(business))
                .map_err(log_error(sub_log))
        }
        _ => Ok(HttpResponse::Unauthorized().json("Unable to verify identity")),
    }
}

/*
#[post("/change_categories")]
pub async fn change_categories(
    category_ids: Json<Vec<Uuid>>,
    state: Data<AppState>
) -> Result<impl Responder, AppError> {
    let category_ids = category_ids.into_inner();
    let db = state.as_ref().db.clone();
}
 */