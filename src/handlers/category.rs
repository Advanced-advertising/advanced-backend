use crate::actors::category::{CreateCategory, GetCategories};
use crate::errors::AppError;
use crate::handlers::log_error;
use crate::models::app_state::AppState;
use crate::models::category::CategoryData;
use actix_web::web::{Data, Json};
use actix_web::{post, get, HttpResponse, Responder};
use slog::o;

#[post("/create")]
pub async fn create(
    category: Json<CategoryData>,
    state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    let db = state.as_ref().db.clone();
    let category = category.into_inner();

    let result = match db
        .send(CreateCategory {
            name: category.category_name,
            logger: state.logger.clone(),
        })
        .await
    {
        Ok(res) => res,
        Err(err) => return Err(AppError::from_mailbox(err)),
    };

    let sub_log = state.logger.new(o!("handle" => "create_category"));
    result
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(log_error(sub_log))
}

#[get("/categories")]
pub async fn get_categories(
    state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    let db = state.as_ref().db.clone();
    let result = match db
        .send(GetCategories)
        .await
    {
        Ok(res) => res,
        Err(err) => return Err(AppError::from_mailbox(err)),
    };

    let sub_log = state.logger.new(o!("handle" => "create_category"));
    result
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(log_error(sub_log))
}
