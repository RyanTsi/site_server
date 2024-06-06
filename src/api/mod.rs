mod post;
mod user;
mod other;

use std::sync::Arc;

use axum::{routing::{get, post}, Router};
use other::handler_chat_list;
use post::{handler_create_post, handler_delete_post, handler_list_postinfo, handler_post_content, handler_remark_list, handler_update_post};
use user::{handler_chat, handler_delete_user, handler_login, handler_register_user, handler_remark, handler_update_user};

use crate::AppState;

pub fn api_route(state: Arc<AppState>) -> Router {
    Router::new()
        .nest("/post", post_route(state.clone()))
        .nest("/user", user_route(state.clone()))
        .route("/chatlist", get(handler_chat_list).with_state(state.clone()))
}

fn post_route(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/create", post(handler_create_post))
        .route("/delete", post(handler_delete_post))
        .route("/update", post(handler_update_post))
        .route("/infolist", get(handler_list_postinfo))
        .route("/remarks", get(handler_remark_list))
        .route("/content", get(handler_post_content))
        .with_state(state)
}

fn user_route(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/register", post(handler_register_user))
        .route("/delete", post(handler_delete_user))
        .route("/update", post(handler_update_user))
        .route("/login", post(handler_login))
        .route("/remark", post(handler_remark))
        .route("/chat", post(handler_chat))
        .with_state(state)
}