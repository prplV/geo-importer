use std::sync::Arc;

use tokio::sync::RwLock;

use crate::{global_state::GlobalState, integr::Integration};

#[derive(Debug, Clone, serde::Deserialize)]
pub(super) struct Query {
    pub(super) lat: f64,
    pub(super) lon: f64,
}

// Arc<RwLock<GlobalState>>
#[derive(Debug, Clone)]
pub(super) struct RouteState {
    pub state: Arc<RwLock<GlobalState>>,
    pub integration: Integration,
}

impl RouteState {
    pub fn new(state: Arc<RwLock<GlobalState>>, integration: Integration) -> Self {
        Self { state, integration }
    }
}
