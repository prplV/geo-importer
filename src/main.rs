mod api;
mod coords;
mod db;
mod geo_task;
mod global_state;
mod integr;
mod log;

use std::sync::Arc;

use tokio::sync::RwLock;
use tracing::info;

use crate::{
    coords::Coordinates, db::get_db_pool, geo_task::start_geo_task, global_state::GlobalState,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let rx = log::init()?;

    let global_state = Arc::new(RwLock::new(GlobalState::new(get_db_pool().await?)));

    let client = global_state.write().await.get_connection().await?;
    let stmt = "SELECT DISTINCT latitude, longitude FROM geos ORDER BY latitude, longitude;";
    let rows = client.query(stmt, &[]).await?;

    //  выгружаем из базы корды, и прихраниваем локально
    for row in rows {
        let (latitude, longitude) = (row.try_get::<_, f64>(0)?, row.try_get::<_, f64>(1)?);
        info!("Latitude: {}, Longitude: {}", latitude, longitude);
        global_state
            .write()
            .await
            .add_target(Coordinates::new(latitude, longitude));
    }

    let task_integr = start_geo_task(global_state.clone(), rx.clone()).await?;
    let api_task = api::start_api(global_state.clone(), rx.clone()).await?;

    let _ = task_integr.await?;
    let _ = api_task.await?;
    Ok(())
}
