use std::sync::Arc;

use axum::{extract::{Multipart, State}, response::IntoResponse, Json};
use tokio::fs::File;

use crate::{error::Result, AppState};

pub async fn handler_chat_list(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse> {
    log::info!("HANDLE {:<12}", "list chat");
    match state.db.list_chat().await {
        Ok(list)  => {
            Ok(Json(list))
        }
        Err(e) => {
            Err(e)
        }
    }
}


pub async fn handler_upload(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse> {
    log::info!("HANDLE {:<12}", "upload");
    let resouse_dir = "./res/";
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let filename = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        let filepath = format!("{}/{}", resouse_dir, filename);

        println!("Length of `{}` is {} bytes", filename, data.len());
        // 创建文件并将图片写入
        if let Ok(mut file) = File::create(filepath).await {

        } else {

        }
        // while let Some(chunk) = field.data().await {
        //     file.write_all(&chunk?)?;
        // }
    }
    Ok(())
}