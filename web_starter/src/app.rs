use crate::{config, id};
use crate::server::Server;
use crate::{database, logger};
use axum::Router;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}
impl AppState {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

pub async fn run(router: Router<AppState>) -> anyhow::Result<()> {
    logger::init();
    id::init()?;
    let db = database::init().await?;
    let state = AppState::new(db);
    Server::new(&config::get().server())
        .start(state, router)
        .await
}
