use std::sync::Arc;

use crate::{global_state::GlobalState, integr::Integration};
use chrono::TimeZone;
use chrono::{Local, Timelike};
use tokio::{select, sync::RwLock, task::JoinHandle, time};
use tracing::{info, trace};

pub type GeoTask = JoinHandle<anyhow::Result<()>>;

pub async fn start_geo_task(
    state: Arc<RwLock<GlobalState>>,
    mut grx: tokio::sync::watch::Receiver<u8>,
) -> anyhow::Result<GeoTask> {
    // Start the geo task
    let integration = Integration::from_env().build()?;

    let task = tokio::spawn(async move {
        trace!("starting geo_task...");
        loop {
            let now = Local::now();
            let next_midnight = if now.hour() == 0 && now.minute() == 0 && now.second() == 0 {
                now + chrono::Duration::days(1)
            } else {
                let naive_next_midnight = (now.date_naive() + chrono::Duration::days(1))
                    .and_hms_opt(0, 0, 0)
                    .ok_or(anyhow::anyhow!("cannot calculate next check time"))?;
                Local.from_local_datetime(&naive_next_midnight).unwrap()
            };

            let until_midnight = next_midnight.signed_duration_since(now);
            // TLDR; comment for debug
            // let until_midnight = chrono::TimeDelta::zero();
            let sleep_duration = time::Duration::from_secs(until_midnight.num_seconds() as u64);

            let sleep_until_midnight = time::sleep_until(time::Instant::now() + sleep_duration);

            select! {
                _ = grx.changed() => {
                    break;
                }
                _ = sleep_until_midnight => {
                    info!("updating geo data, starting daily routine ...");
                    let ref_state = state.read().await;
                    let mut conn = ref_state.get_connection().await?;
                    let targets = ref_state.get_targets().clone();
                    drop(ref_state);
                    let current_date = Local::now().date_naive();

                    let trns = conn.transaction().await?;

                    for target in targets {
                        let lat = target.get_latitude();
                        let lon = target.get_longitude();

                        let r_builder = integration.get_request();
                        let r_builder = r_builder.query(&[("lat", lat.to_string()), ("lon", lon.to_string())]);

                        let response = r_builder.send().await?;
                        let data: serde_json::Value = response.json().await?;

                        let stmt = r#"INSERT INTO geos (latitude, longitude, date, data)
                        VALUES ($1, $2, $3, $4);"#;

                        trns.execute(stmt, &[&lat, &lon, &current_date, &data]).await?;
                    }

                    trns.commit().await?;
                    time::sleep(time::Duration::from_secs(1)).await;
                }
            }
        }
        trace!("end of geo_task");
        Ok(())
    });

    Ok(task)
}
