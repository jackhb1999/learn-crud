use crate::app::AppState;
use crate::common::{Page, PaginationParams};
use crate::entity::prelude::User;
use crate::entity::user;
use crate::error::ApiResult;
use crate::response::ApiResponse;
use anyhow::Context;
use axum::Router;
use axum::extract::State;
use axum::routing::get;

use crate::query::Query;
use crate::valid::{Valid, ValidQuery};
use sea_orm::prelude::*;
use sea_orm::{Condition, EntityTrait, QueryOrder, QueryTrait};
use serde::Deserialize;
use validator::Validate;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/", get(query_users))
        .route("/page", get(find_page))
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UserQueryParams {
    pub keyword: Option<String>,
    #[validate(nested)]
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

#[tracing::instrument(name = "Query users", skip_all)]
#[axum::debug_handler]
async fn query_users(
    State(AppState { db }): State<AppState>,
) -> ApiResult<ApiResponse<Vec<user::Model>>> {
    tracing::info!("开始处理：查询用户");
    let users = User::find()
        .filter(user::Column::UserCode.eq("jar"))
        .all(&db)
        .await
        .context("Failed to query users")?;
    Ok(ApiResponse::success(Some(users)))
}

///
/// 使用axum自带的 Query 提取器到手动实现
///async fn find_page_(
//     State(AppState { db }): State<AppState>,
//     Query(UserQueryParams {
//         keyword,
//         pagination,
//     }): Valid<Query<UserQueryParams>>,
// ) -> ApiResult<ApiResponse<Page<user::Model>>> {
///
/// 实现参数校验
#[axum::debug_handler]
async fn find_page_(
    State(AppState { db }): State<AppState>,
    Valid(Query(UserQueryParams {
        keyword,
        pagination,
    })): Valid<Query<UserQueryParams>>,
) -> ApiResult<ApiResponse<Page<user::Model>>> {
    let paginator = User::find()
        .apply_if(keyword.as_ref(), |query, keyword| {
            query.filter(
                Condition::any()
                    .add(user::Column::Username.contains(keyword))
                    .add(user::Column::UserCode.contains(keyword)),
            )
        })
        .order_by_desc(user::Column::Id)
        .paginate(&db, pagination.page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(pagination.page - 1).await?;
    let page = pagination.to_page(total, items);

    Ok(ApiResponse::success(Some(page)))
}

#[axum::debug_handler]
async fn find_page(
    State(AppState { db }): State<AppState>,
    ValidQuery(UserQueryParams {
        keyword,
        pagination,
    }): ValidQuery<UserQueryParams>,
) -> ApiResult<ApiResponse<Page<user::Model>>> {
    let paginator = User::find()
        .apply_if(keyword.as_ref(), |query, keyword| {
            query.filter(
                Condition::any()
                    .add(user::Column::Username.contains(keyword))
                    .add(user::Column::UserCode.contains(keyword)),
            )
        })
        .order_by_desc(user::Column::Id)
        .paginate(&db, pagination.page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(pagination.page - 1).await?;
    let page = pagination.to_page(total, items);

    Ok(ApiResponse::success(Some(page)))
}
