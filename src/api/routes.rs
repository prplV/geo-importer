use axum::{
    Json,
    extract::{Query, State},
    response::IntoResponse,
};
use reqwest::StatusCode;

use crate::coords::Coordinates;

use super::data::{Query as Params, RouteState};

//  GET
pub async fn fetch_tracked(
    State(state): State<RouteState>,
    // Query(params): Query<Params>,
) -> impl IntoResponse {
    Json(state.state.read().await.get_targets().clone())
}

//  DELETE
pub async fn delete_tracked(
    State(state): State<RouteState>,
    Query(params): Query<Params>,
) -> impl IntoResponse {
    if state
        .state
        .write()
        .await
        .remove_target(&Coordinates::new(params.lat, params.lon))
    {
        (StatusCode::OK, "Target was deleted")
    } else {
        (StatusCode::NOT_MODIFIED, "Target not found")
    }
}

//  POST
pub async fn add_tracked(
    State(state): State<RouteState>,
    Query(params): Query<Params>,
) -> impl IntoResponse {
    let r_state = state.state.read().await;

    if r_state
        .get_targets()
        .contains(&Coordinates::new(params.lat, params.lon))
    {
        return (StatusCode::NOT_MODIFIED, "Target was found").into_response();
    }
    drop(r_state);

    let mut w_state = state.state.write().await;
    w_state.add_target(Coordinates::new(params.lat, params.lon));

    let current_date = chrono::Local::now().date_naive();

    match state
        .integration
        .get_request()
        .query(&[
            ("lat", params.lat.to_string()),
            ("lon", params.lon.to_string()),
        ])
        .send()
        .await
    {
        Ok(response) => {
            // response
            // (StatusCode::INTERNAL_SERVER_ERROR, "err".to_string()).into_response()
            match response.json::<serde_json::Value>().await {
                Ok(data) => {
                    // Process the data
                    let connection = if let Ok(connection) = w_state.get_connection().await {
                        connection
                    } else {
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "Failed to get connection",
                        )
                            .into_response();
                    };

                    let stmt = r#"INSERT INTO geos (latitude, longitude, date, data)
                    VALUES ($1, $2, $3, $4);"#;

                    let result = connection
                        .execute(stmt, &[&params.lat, &params.lon, &current_date, &data])
                        .await;

                    match result {
                        Ok(_) => (StatusCode::CREATED, "Target was added").into_response(),
                        Err(err) => {
                            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
                        }
                    }
                }
                Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
            }
        }
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
    // .json::<serde_json::Value>()
    // .await?
}
