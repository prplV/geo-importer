use std::collections::HashSet;

use deadpool_postgres::Pool;

use crate::coords::Coordinates;

#[derive(Debug)]
pub struct GlobalState {
    pool: Pool,
    targets: HashSet<Coordinates>,
}

#[allow(dead_code)]
impl GlobalState {
    pub fn new(pool: Pool) -> Self {
        Self {
            pool,
            targets: HashSet::new(),
        }
    }

    // targets
    pub fn get_targets(&self) -> &HashSet<Coordinates> {
        &self.targets
    }

    pub fn add_target(&mut self, target: Coordinates) {
        self.targets.insert(target);
    }

    pub fn remove_target(&mut self, target: &Coordinates) -> bool {
        self.targets.remove(target)
    }

    // pool
    pub async fn get_connection(&self) -> anyhow::Result<deadpool_postgres::Client> {
        self.pool.get().await.map_err(|er| anyhow::anyhow!(er))
    }
}
