use crate::actors::category::{CreateCategory, GetAllCategories, UpdateCategory};
use crate::errors::AppError;
use crate::handlers::log_error;
use crate::models::app_state::AppState;
use crate::models::category::{Category, CategoryData};
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
        .map(|category| HttpResponse::Ok().json(category))
        .map_err(log_error(sub_log))
}

#[post("/update")]
pub async fn update(
    category: Json<Category>,
    state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    let db = state.as_ref().db.clone();
    let category = category.into_inner();

    let result = match db
        .send(UpdateCategory {
            id: category.category_id,
            name: category.category_name,
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
pub async fn get_categories(
    state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    let db = state.as_ref().db.clone();
    let result = match db
        .send(GetAllCategories)
        .await
    {
        Ok(res) => res,
        Err(err) => return Err(AppError::from_mailbox(err)),
    };

    let sub_log = state.logger.new(o!("handle" => "create_category"));
    result
        .map(|categories| HttpResponse::Ok().json(categories))
        .map_err(log_error(sub_log))
}

