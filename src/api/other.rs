use std::sync::Arc;

use axum::{extract::{Multipart, State}, response::IntoResponse, Json};
use tokio::{fs::File, io::AsyncWriteExt};

use crate::{config::{self, PartialConfig}, error::{Error, Result}, AppState};


pub async fn handler_basic_info(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse> {
    log::info!("HANDLE {:<12}", "basic info");
    // let config = state.config.lock().unwrap();
    Ok(Json(state.config.clone()))
}

pub async fn handler_update_basic_info(
    State(state): State<Arc<AppState>>,
    params: Json<PartialConfig>,
) -> Result<impl IntoResponse> {
    log::info!("HANDLE {:<12}", "update basic info");
    let mut config = state.config.lock().unwrap();
    let new_config = PartialConfig {sitebasicinfo: params.sitebasicinfo.clone(), mylinks: params.mylinks.clone()};
    config.update(new_config);
    
    Ok(())
}

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
    mut multipart: Multipart,
) -> Result<impl IntoResponse> {
    log::info!("HANDLE {:<12}", "upload");
    let resouse_dir = "./res/";
    while let Some(field) = multipart.next_field().await.unwrap() {
        let filename = field.name().unwrap().to_string();
        log::debug!("{}", &filename);
        let data = field.bytes().await.unwrap();
        let filepath = format!("{}/{}", resouse_dir, filename);

        println!("Length of `{}` is {} bytes", filename, data.len());
        // 创建文件并将图片写入
        if let Ok(mut file) = File::create(filepath).await {
            file.write_all(&data).await?;
        } else {
            return Err(Error::IOError)
        }
    }
    Ok(())
}