use std::sync::Arc;

use axum::{extract::{Query, State}, response::IntoResponse, Json};
use serde::Deserialize;

use crate::{db::TagName, error::Result, model::{ClientPostInfo, PostInfo}, utils::get_uuid, AppState, Uuid};

#[derive(Deserialize)]
pub struct CreatePostParams {
    title: String,
    author: u32,
    brief: String,
    content: String,
    tags: Vec<String>,
}

#[derive(Deserialize)]
pub struct DeletePostParams {
    pid: Uuid,
}

pub async fn handler_create_post (
    State(state): State<Arc<AppState>>,
    params: Json<CreatePostParams>,
) -> Result<impl IntoResponse> {
    log::info!("HANDLE {:<12}", "create post");
    let pid = get_uuid();
    match state.db.push_post(&pid, &params.title, params.author, &params.brief, &params.content, &params.tags).await {
        Ok(_) => {
            Ok(())
        }
        Err(e) => {
            return Err(e);
        }
    }
}

pub async fn handler_delete_post (
    State(state): State<Arc<AppState>>,
    params: Json<DeletePostParams>,
) -> Result<impl IntoResponse> {
    let pid = &params.pid;
    log::info!("HANDLE {:<12}", "delete_post");
    match state.db.delete_post(pid).await {
        Ok(_) => {
            Ok(())
        },
        Err(e) => {
            return Err(e);
        },
    }
}

#[derive(Deserialize)]
pub struct UpdatePostParams {
    pid: Uuid,
    title: String,
    brief: String,
    content: String,
    tags: Vec<String>,
}

pub async fn handler_update_post (
    State(state): State<Arc<AppState>>,
    params: Json<UpdatePostParams>,
) -> Result<impl IntoResponse> {
    log::info!("HANDLE {:<12}", "update post");
    match state.db.update_post(&params.pid, &params.title, &params.brief, &params.content, &params.tags).await {
        Ok(_) => {
            Ok(())
        }
        Err(e) => {
            return Err(e);
        }
    }
}

pub async fn handler_list_postinfo (
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse> {
    log::info!("HANDLE {:<12}", "list post");
    let postinfos = state.db.get_post_list().await?;
    let mut posts = Vec::new();
    for p in postinfos {
        let tags = state.db.get_post_alltag(&p.pid).await?;
        posts.push(package_tag(p, tags).await);
    }
    Ok(Json(posts))
}

pub async fn package_tag(
    postinfos: PostInfo,
    tags: Vec<TagName>,
) -> ClientPostInfo {
    let mut res = ClientPostInfo::from(postinfos);
    for tag in tags {
        res.add_tag(tag);
    }
    res
}

#[derive(Deserialize)]
pub struct PidParams {
    pid: Uuid,
}

pub async fn handler_post_content(
    State(state): State<Arc<AppState>>,
    Query(query): Query<PidParams>
) -> Result<impl IntoResponse> {
    log::info!("HANDLE {:<12}", "post content");
    match state.db.get_post_conent(&query.pid).await {
        Ok(content) => {
            Ok(Json(content))
        }
        Err(e) => {
            Err(e)
        }
    }
}


pub async fn handler_remark_list(
    State(state): State<Arc<AppState>>,
    Query(query): Query<PidParams>
) -> Result<impl IntoResponse> {
    log::info!("HANDLE {:<12}", "list remark");
    match state.db.list_remark(&query.pid).await {
        Ok(list)  => {
            Ok(Json(list))
        }
        Err(e) => {
            Err(e)
        }
    }
}