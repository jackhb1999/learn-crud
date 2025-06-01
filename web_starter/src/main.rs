mod config;
mod database;
mod entity;
mod logger;

use axum::extract::State;
use axum::response::IntoResponse;
use axum::{Router, routing};
use entity::prelude::*;
use sea_orm::prelude::*;
use tokio::net::TcpListener;
use crate::entity::user;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init();
    let db = database::init().await?;
    let router = Router::new()
        .route("/", routing::get(index))
        .route("/user", routing::get(query_users))
        .with_state(db);
    let port = config::get().server().port();
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();
    tracing::info!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, router).await.unwrap();
    Ok(())
}

#[axum::debug_handler]
async fn index() -> &'static str {
    "Hello, World!"
}

#[axum::debug_handler]
async fn query_users(State(db): State<DatabaseConnection>) -> impl IntoResponse {
    let users = User::find().filter(user::Column::UserCode.eq("jar")).all(&db).await.unwrap();
    axum::Json(users)
}
