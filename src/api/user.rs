use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{error::Result, model::User, AppState, Uuid};

#[derive(Deserialize)]
pub struct RegisterUserParams {
    name: String,
    passwd: String,
}

pub async fn handler_register_user (
    State(state): State<Arc<AppState>>,
    params: Json<RegisterUserParams>,
) -> Result<impl IntoResponse> {
    log::info!("HANDLE {:<12}", "register user");
    match state.db.register_user(&params.name, &params.passwd).await {
        Ok(uid) => {
            Ok(Json(uid))
        }
        Err(e) => {
            Err(e)
        }
    }
}

#[derive(Deserialize)]
pub struct DeleteUserParams {
    uid: u32,
}

pub async fn handler_delete_user (
    State(state): State<Arc<AppState>>,
    params: Json<DeleteUserParams>
) -> Result<()> {
    log::info!("HANDLE {:<12}", "delete user");
    match state.db.delete_user(params.uid).await {
        Ok(_) => {
            Ok(())
        }
        Err(e) => {
            Err(e)
        }
    }
}

#[derive(Deserialize)]
pub struct UpdateUserParams {
    uid: u32,
    passwd: String,
    name: String,
    avatar: Option<String>,
    bio: Option<String>,
}

pub async fn handler_update_user (
    State(state): State<Arc<AppState>>,
    params: Json<UpdateUserParams>,
) -> Result<impl IntoResponse> {
    log::info!("HANDLE {:<12}", "update user");
    match state.db.update_user(params.uid, &params.passwd, &params.name, &params.avatar, &params.bio).await {
        Ok(_) => {
            Ok(())
        }
        Err(e) => {
            Err(e)
        }
    }
}

#[derive(Deserialize)]
pub struct LoginParams {
    uid: u32,
    passwd: String,
}

#[derive(Serialize)]
pub struct LoginInfo {
    code: u32,
    userinfo: Option<User>,
}
impl LoginInfo {
    fn new(c: u32, user: Option<User>) -> Self {
        Self {
            code: c,
            userinfo: user,
        }
    }
}

pub async fn handler_login (
    State(state): State<Arc<AppState>>,
    params: Json<LoginParams>,
) -> Result<impl IntoResponse> {
    log::info!("HANDLE {:<12}", "login");
    match state.db.check_passwd(params.uid, &params.passwd).await {
        Ok(r) => {
            if r {
                Ok(Json(LoginInfo::new(1, Some(state.db.get_user_info(params.uid).await?))))
            } else {
                Ok(Json(LoginInfo::new(0, None)))
            }
        }
        Err(e) => {
            Err(e)
        }
    }
}

#[derive(Deserialize)]
pub struct RemarkParams {
    pid: Uuid,
    uid: u32,
    content: String,
}

pub async fn handler_remark(
    State(state): State<Arc<AppState>>,
    params: Json<RemarkParams>,
) -> Result<impl IntoResponse> {
    log::info!("HANDLE {:<12}", "remark");
    state.db.push_remark(params.uid, &params.pid, &params.content).await?;
    Ok(())
}


#[derive(Deserialize)]
pub struct ChatParams {
    uid: u32,
    content: String,
}

pub async fn handler_chat(
    State(state): State<Arc<AppState>>,
    params: Json<ChatParams>,
) -> Result<impl IntoResponse> {
    log::info!("HANDLE {:<12}", "chat");
    state.db.push_chatmsg(params.uid, &params.content).await?;
    Ok(())
}