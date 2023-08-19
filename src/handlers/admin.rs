use crate::actors::address::CreateAddress;
use crate::actors::admin::{AuthorizeAdmin, CreateAdmin};
use crate::actors::screens::CreateScreen;
use crate::errors::AppError;
use crate::handlers::log_error;
use crate::models::address::AddressData;
use crate::models::app_state::AppState;
use crate::models::screen::ScreenData;
use crate::models::user::UserData;
use actix_web::web::{Data, Json};
use actix_web::{get, post, HttpResponse, Responder};
use actix_web_httpauth::extractors::basic::BasicAuth;
use slog::o;

#[post("/create")]
pub async fn register(
    user: Json<UserData>,
    state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    let db = state.as_ref().db.clone();
    let user = user.into_inner();

    let result = match db
        .send(CreateAdmin {
            name: user.user_name,
            password: user.password,
            logger: state.logger.clone(),
        })
        .await
    {
        Ok(res) => res,
        Err(err) => return Err(AppError::from_mailbox(err)),
    };

    let sub_log = state.logger.new(o!("handle" => "create_admin"));
    result
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(log_error(sub_log))
}

#[get("/login")]
pub async fn login(
    basic_auth: BasicAuth,
    state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    let db = state.as_ref().db.clone();

    let authorise_user = AuthorizeAdmin { basic_auth };

    let result = match db.send(authorise_user).await {
        Ok(res) => res,
        Err(err) => return Err(AppError::from_mailbox(err)),
    };

    let sub_log = state.logger.new(o!("handle" => "login admin"));

    result
        .map(|token_str| HttpResponse::Ok().json(token_str))
        .map_err(log_error(sub_log))
}

#[post("/create_address")]
pub async fn create_address(
    address_data: Json<AddressData>,
    state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    let db = state.as_ref().db.clone();
    let address_data = address_data.into_inner();

    let result = match db
        .send(CreateAddress {
            name: address_data.address_name,
            business_id: address_data.business_id,
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

#[post("/create_screen")]
pub async fn create_screen(
    screen_data: Json<ScreenData>,
    state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    let db = state.as_ref().db.clone();
    let screen_data = screen_data.into_inner();

    let result = match db
        .send(CreateScreen {
            name: screen_data.screen_name,
            price_per_time: screen_data.price_per_time,
            characteristics: screen_data.characteristics,
            traffic: screen_data.traffic,
            business_id: screen_data.business_id,
            address_id: screen_data.address_id,
            logger: state.logger.clone(),
        })
        .await
    {
        Ok(res) => res,
        Err(err) => return Err(AppError::from_mailbox(err)),
    };

    let sub_log = state.logger.new(o!("handle" => "create_screen"));
    result
        .map(|screen| HttpResponse::Ok().json(screen))
        .map_err(log_error(sub_log))
}
