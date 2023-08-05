use crate::actors::business::{AuthorizeBusiness, ChangeImg, CreateBusiness, GetAllBusinesses};
use crate::errors::{AppError, AppErrorType};
use crate::files::save_files;
use crate::handlers::log_error;
use crate::middleware::token::TokenClaims;
use crate::models::app_state::AppState;
use crate::models::business::BusinessData;
use actix_multipart::Multipart;
use actix_web::web::{Data, Json, ReqData};
use actix_web::{get, post, HttpResponse, Responder};
use actix_web_httpauth::extractors::basic::BasicAuth;
use slog::o;

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
            working_time: business.working_time,
            password: business.password,
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
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(log_error(sub_log))
}

#[get("/login")]
pub async fn login(
    basic_auth: BasicAuth,
    state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    let password = match basic_auth.password() {
        Some(pass) => pass,
        None => {
            return Err(AppError {
                message: Some("Must provide username and password".to_string()),
                cause: None,
                error_type: AppErrorType::SomethingWentWrong,
            })
        }
    };

    let authorise_business = AuthorizeBusiness {
        name: basic_auth.user_id().to_string(),
        password: password.into(),
    };

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
            let img_url = match save_files(payload).await {
                Ok(paths) => match paths.get(0) {
                    None => return Ok(HttpResponse::BadRequest().body("failed to upload file")),
                    Some(path) => path.clone(),
                },
                _ => return Ok(HttpResponse::BadRequest().body("failed to upload file")),
            };

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
