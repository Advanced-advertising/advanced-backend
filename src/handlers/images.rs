use crate::errors::AppError;
use crate::handlers::log_io_error;
use crate::middleware::token::TokenClaims;
use crate::models::app_state::AppState;
use actix_multipart::Multipart;
use actix_web::web::{Data, ReqData};
use actix_web::{get, web, HttpResponse};
use futures_util::{StreamExt, TryStreamExt};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use slog::{info, o};
use std::io::Write;
use std::path::Path;
use uuid::Uuid;

#[get("")]
async fn get_image(
    img_url: String,
    req: Option<ReqData<TokenClaims>>,
    state: Data<AppState>,
) -> Result<HttpResponse, AppError> {
    return Ok(HttpResponse::Ok().finish());
    if !Path::new(&img_url).exists() {
        info!(state.logger, "NO");
        info!(state.logger, "{}", img_url.to_string());
    } else {
        info!(state.logger, "OK");
    }

    let url = Path::new("media").join(img_url);
    let result = tokio::fs::read(&url).await;

    let sub_log = state.logger.new(o!("handle" => "get image"));

    result
        .map(|image_bytes| {
            let content_type = "image/png";
            HttpResponse::Ok()
                .content_type(content_type)
                .body(image_bytes)
        })
        .map_err(log_io_error(sub_log))
}

pub async fn save_files(mut payload: Multipart) -> Result<String, AppError> {
    let mut paths = Vec::new();
    while let Ok(Some(mut field)) = payload.try_next().await {
        let file_name = get_random_file_name();
        let file_path = format!("{}{}.png", "media/", file_name);
        paths.push(file_path.clone());

        let mut f = web::block(|| std::fs::File::create(file_path))
            .await
            .unwrap()?;

        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f = web::block(move || f.write_all(&data).map(|_| f))
                .await
                .unwrap()?;
        }
    }

    Ok(paths.get(0).unwrap().clone())
}

fn get_random_file_name() -> String {
    let file_name = Uuid::new_v4().to_string();
    utf8_percent_encode(&file_name, NON_ALPHANUMERIC).to_string()
}
