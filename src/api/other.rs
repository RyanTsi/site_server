use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};

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
    State(state): State<Arc<AppState>>
) -> Result<impl IntoResponse> {
    Ok(())
}