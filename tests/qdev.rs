#![allow(unused)]

use std::{f32::consts::PI, ptr::null};

use anyhow::Result;
use axum::Json;
use serde::de;
use serde_json::json;

async fn post_test() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8300")?;
    let req_create_post = hc.do_post(
        "/api/post/create", 
        json!({
            "title": "test",
            "author" : 100000,
            "brief": "广告招租位",
            "content" : "test",
            "tags": ["广告", "测试"]
        })
    );

    req_create_post.await?.print().await?;

    let req_delete_post = hc.do_post(
        "/api/post/delete", 
        json!({
            "pid": "f354c332-a2ad-42a6-bdae-99b535395ef1",
        })
    );

    req_delete_post.await?.print().await?;

    let req_update_post = hc.do_post(
        "/api/post/update", 
        json!({
            "pid": "dfebd659-9cd2-4c9a-8a42-85fbf561bd9a",
            "title": "update",
            "brief": "广告招租位update",
            "content": "更新！！！！",
            "tags": ["广告", "测试", "更新"]
        })
    );

    req_update_post.await?.print().await?;

    let req_list_post = hc.do_get(
        "/api/post/infolist"
    );
    req_list_post.await?.print().await?;

    let get_content = hc.do_get(
        "/api/post/content?pid=dfebd659-9cd2-4c9a-8a42-85fbf561bd9a"
    );
    get_content.await?.print().await?;
    Ok(())
}


async fn user_test() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8300")?;
    let req_register_user = hc.do_post(
        "/api/user/register", 
        json!({
            "name": "root",
            "passwd":  "root",
        })
    );
    req_register_user.await?.print().await?;

    let delete_user = hc.do_post(
        "/api/user/delete", 
        json!({
            "uid": 10000002
        })
    );
    delete_user.await?.print().await?;

    let update_user = hc.do_post(
        "/api/user/update", 
        json!({
            "uid": 10000000,
            "passwd": "root",
            "name": "root",
            "avatar": null,
            "bio": null,
        })
    );
    update_user.await?.print().await?;
    
    let login_bad = hc.do_post(
        "/api/user/login",
        json!({
            "uid": 10000000,
            "passwd": "toot",
        })
    );
    login_bad.await?.print().await?;
    
    let login_notfound = hc.do_post(
        "/api/user/login",
        json!({
            "uid": 1000,
            "passwd": "toot",
        })
    );
    login_notfound.await?.print().await?;

    let login_ok = hc.do_post(
        "/api/user/login",
        json!({
            "uid": 10000000,
            "passwd": "root",
        })
    );
    login_ok.await?.print().await?;

    Ok(())
}


async fn meg_test() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8300")?;
    let remark = hc.do_post(
        "/api/user/remark", 
        json!({
            "uid": 10000000,
            "pid": "dfebd659-9cd2-4c9a-8a42-85fbf561bd9a",
            "content": "remark",
        })
    );
    remark.await?.print().await?;
    let remark_list = hc.do_get(
        "/api/post/remarks?pid=dfebd659-9cd2-4c9a-8a42-85fbf561bd9a"
    );
    remark_list.await?.print().await?;

    let chat_list = hc.do_get(
        "/api/chatlist"
    );
    chat_list.await?.print().await?;

    let chat = hc.do_post(
        "/api/user/chat", 
        json!({
            "uid": 10000000,
            "content": "chat",
        })
    );
    chat.await?.print().await?;
    Ok(())
}


#[tokio::test]
async fn qdev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8300")?;

    hc.do_get("/").await?.print().await?;

    post_test().await?;

    meg_test().await?;

    user_test().await?;
    Ok(())
}