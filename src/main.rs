use std::sync::Arc;

use axum::{routing::get, Router};
use site_server::{api::api_route, error::Result, AppState};


async fn initialize() -> Result<Arc<AppState>> {
    dotenv::dotenv().ok();
    env_logger::init();
    AppState::new().await
}

#[tokio::main]
async fn main () -> Result<()> {
    let state = initialize().await?;
    
    let app = Router::new()
        .route("/", get(|| async {"running..."}))
        .nest("/api", api_route(state));

        let listener = tokio::net::TcpListener::bind("0.0.0.0:8210").await.unwrap();
        log::info!("LISTENING on {:?}\n", listener.local_addr());

        axum::serve(listener, app).await.unwrap();
    
    Ok(())
}

