use actix_multipart::Multipart;
use actix_web::web;
use futures_util::{StreamExt, TryStreamExt};
use std::io::Write;
use uuid::Uuid;

pub async fn save_files(mut payload: Multipart) -> Result<Vec<String>, std::io::Error> {
    let mut paths = Vec::new();
    while let Ok(Some(mut field)) = payload.try_next().await {
        let file_name = Uuid::new_v4().to_string();
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

    Ok(paths)
}
