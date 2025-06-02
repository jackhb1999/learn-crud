use crate::app::AppState;
use crate::entity::prelude::User;
use crate::entity::user;
use crate::error::ApiResult;
use crate::response::ApiResponse;
use anyhow::Context;
use axum::Router;
use axum::extract::State;
use axum::routing::get;
use sea_orm::EntityTrait;
use sea_orm::prelude::*;

pub fn create_router() -> Router<AppState> {
    Router::new().route("/", get(query_users))
}

#[axum::debug_handler]
async fn query_users(
    State(AppState { db }): State<AppState>,
) -> ApiResult<ApiResponse<Vec<user::Model>>> {
    let users = User::find()
        .filter(user::Column::UserCode.eq("jar"))
        .all(&db)
        .await
        .context("Failed to query users")?;
    Ok(ApiResponse::success(Some(users)))
}
