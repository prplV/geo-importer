mod data;
mod routes;

use std::sync::Arc;

use axum::{
    Router,
    routing::{delete, get, post},
};
use tokio::{sync::RwLock, task::JoinHandle};
use tracing::trace;

use crate::{api::data::RouteState, global_state::GlobalState, integr::Integration};

pub async fn start_api(
    state: Arc<RwLock<GlobalState>>,
    mut grx: tokio::sync::watch::Receiver<u8>,
) -> anyhow::Result<JoinHandle<anyhow::Result<()>>> {
    let serve_addr = format!(
        "0.0.0.0:{}",
        std::env::var("SELF_PORT")
            .inspect_err(|_| tracing::trace!("defaulting api port..."))
            .unwrap_or("8080".to_string())
    );

    let integration = Integration::from_env().build()?;
    let state = RouteState::new(state, integration);

    let router = Router::new().route(
        "/place",
        get(routes::fetch_tracked)
            .delete(routes::delete_tracked)
            .post(routes::add_tracked),
    );

    let router = Router::new().nest("/api/v1", router).with_state(state);

    let listen = tokio::net::TcpListener::bind(serve_addr).await?;

    let task = tokio::spawn(async move {
        trace!("starting api task ...");
        tokio::select! {
            res = axum::serve(listen, router) => {
                if let Err(e) = res {
                    tracing::error!("api task failed: {}", e);
                }
            },
            _ = grx.changed() => {}
        }
        trace!("end of api task");
        Ok(())
    });

    Ok(task)
}
