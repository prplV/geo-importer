use deadpool_postgres::{Config, Pool, Runtime};
use tokio_postgres::NoTls;
use tracing::trace;

refinery::embed_migrations!("./migrations");

#[derive(Debug, serde::Deserialize)]
struct MyConfig {
    pg: Config,
}

impl MyConfig {
    pub fn from_env() -> Self {
        let mut pg = Config::new();
        pg.host = std::env::var("PG__HOST").ok();
        pg.port = std::env::var("PG__PORT")
            .ok()
            .map(|port| port.parse().unwrap_or(5432));
        pg.user = std::env::var("PG__USER").ok();
        pg.password = std::env::var("PG__PASSWORD").ok();
        pg.dbname = std::env::var("PG__DBNAME").ok();

        MyConfig { pg }
    }

    pub fn get_pool(&self) -> anyhow::Result<Pool> {
        self.pg
            .create_pool(Some(Runtime::Tokio1), NoTls)
            .map_err(|er| anyhow::anyhow!(er))
    }
}

pub async fn get_db_pool() -> anyhow::Result<Pool> {
    let pool = MyConfig::from_env().get_pool()?;
    pool.resize(20);
    trace!("Database pool initialized");

    let mut client = pool.get().await?;

    migrations::runner().run_async(&mut **client).await?;

    Ok(pool)
}
