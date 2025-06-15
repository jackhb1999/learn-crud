use crate::config;
use anyhow::anyhow;
use sea_orm::{
    ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbBackend, Statement,
};
use std::cmp::max;
use tracing::log;

pub async fn init() -> anyhow::Result<DatabaseConnection> {
    let database_config = &config::get().database();

    let mut options = ConnectOptions::new(format!(
        "postgres://{}:{}@{}:{}/{}",
        database_config.user(),
        database_config.password(),
        database_config.host(),
        database_config.port(),
        database_config.database(),
    ));
    let cpuCount = num_cpus::get() as u32;
    options
        .max_connections(max(cpuCount * 4, 20))
        .min_connections(max(cpuCount * 2, 10))
        .connect_timeout(std::time::Duration::from_secs(10))
        .idle_timeout(std::time::Duration::from_secs(300))
        .max_lifetime(std::time::Duration::from_secs(36000 * 24))
        .acquire_timeout(std::time::Duration::from_secs(30))
        .sqlx_logging(false)
        .set_schema_search_path(database_config.schema());

    let db = Database::connect(options).await?;
    db.ping().await?;
    tracing::info!(
        "Database connection established: {:?}",
        db.get_database_backend()
    );
    log_database_version(&db).await?;
    Ok(db)
}

async fn log_database_version(db: &DatabaseConnection) -> anyhow::Result<()> {
    let version = db
        .query_one(Statement::from_string(
            DbBackend::Postgres,
            String::from("SELECT version()"),
        ))
        .await?
        .ok_or_else(|| anyhow!("database version not found"))?;

    tracing::info!(
        "Database version: {}",
        version.try_get_by_index::<String>(0)?
    );
    Ok(())
}
