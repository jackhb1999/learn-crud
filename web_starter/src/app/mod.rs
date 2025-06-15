use crate::{config};

use axum::Router;
use sea_orm::DatabaseConnection;
use crate::app::server::Server;

pub mod error;
mod logger;
 pub mod response;
mod server;
mod latency;
pub mod common;
mod serde;
pub mod query;
pub mod path;
mod json;
pub mod valid;
mod validation;
pub mod id;
mod enumeration;

mod database;
pub mod auth;
pub mod middleware;
pub mod utils;

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
