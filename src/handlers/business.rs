use crate::actors::business::{
    AuthorizeBusiness, ChangeBusinessInfo, ChangeImg, CreateBusiness, GetAllBusinesses,
    GetBusinessCategories, GetBusinessesInfo,
};
use crate::errors::AppError;
use crate::handlers::images::save_files;
use crate::handlers::log_error;
use crate::middleware::token::TokenClaims;
use crate::models::app_state::AppState;
use crate::models::business::{BusinessData, BusinessInfo};
use actix_multipart::Multipart;
use actix_web::web::{Data, Json, ReqData};
use actix_web::{get, post, HttpResponse, Responder};
use actix_web_httpauth::extractors::basic::BasicAuth;
use slog::o;
use uuid::Uuid;

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

#[post("/get_business_info")]
pub async fn get_business_info(
    req: Option<ReqData<TokenClaims>>,
    state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    match req {
        Some(business) => {
            let db = state.as_ref().db.clone();
            let result = match db
                .send(GetBusinessesInfo {
                    business_id: business.id,
                    logger: state.logger.clone(),
                })
                .await
            {
                Ok(res) => res,
                Err(err) => return Err(AppError::from_mailbox(err)),
            };

            let sub_log = state.logger.new(o!("handle" => "get_business_info"));
            result
                .map(|business_info| HttpResponse::Ok().json(business_info))
                .map_err(log_error(sub_log))
        }
        _ => Ok(HttpResponse::Unauthorized().json("Unable to verify identity")),
    }
}

#[post("/get_business_info_by_id")]
pub async fn get_business_info_by_id(
    business_id: Json<Uuid>,
    state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    let db = state.as_ref().db.clone();
    let business_id = business_id.into_inner();
    let result = match db
        .send(GetBusinessesInfo {
            business_id,
            logger: state.logger.clone(),
        })
        .await
    {
        Ok(res) => res,
        Err(err) => return Err(AppError::from_mailbox(err)),
    };

    let sub_log = state.logger.new(o!("handle" => "get_business_info_by_id"));
    result
        .map(|business_info| HttpResponse::Ok().json(business_info))
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

#[post("/change_business_info")]
pub async fn change_business_info(
    business_info: Json<BusinessInfo>,
    state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    let db = state.as_ref().db.clone();
    let business_info = business_info.into_inner();

    let result = match db
        .send(ChangeBusinessInfo {
            business_info,
            logger: state.logger.clone(),
        })
        .await
    {
        Ok(res) => res,
        Err(err) => return Err(AppError::from_mailbox(err)),
    };

    let sub_log = state.logger.new(o!("handle" => "change_categories"));
    result
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(log_error(sub_log))
}

#[post("/get_categories")]
pub async fn get_categories(
    business_id: Json<Uuid>,
    state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    let db = state.as_ref().db.clone();
    let business_id = business_id.into_inner();
    let result = match db
        .send(GetBusinessCategories {
            business_id,
            logger: state.logger.clone(),
        })
        .await
    {
        Ok(res) => res,
        Err(err) => return Err(AppError::from_mailbox(err)),
    };

    let sub_log = state.logger.new(o!("handle" => "change_categories"));
    result
        .map(|categories| HttpResponse::Ok().json(categories))
        .map_err(log_error(sub_log))
}
