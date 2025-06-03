use crate::app::AppState;
use crate::common::{Page, PaginationParams};
use crate::entity::prelude::User;
use crate::entity::user;
use crate::error::{ApiError, ApiResult};
use crate::response::ApiResponse;
use anyhow::Context;
use axum::Router;
use axum::extract::State;
use axum::routing::{get, put};

use crate::entity::user::ActiveModel;
use crate::path::Path;
use crate::query::Query;
use crate::valid::{Valid, ValidJson, ValidQuery};
use sea_orm::prelude::*;
use sea_orm::{ActiveValue, Condition, EntityTrait, IntoActiveModel, QueryOrder, QueryTrait};
use serde::Deserialize;
use validator::Validate;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/", get(query_users))
        .route("/page", get(find_page))
        .route("/update/{id}", put(update))
        .route("/delete/{id}", get(delete))
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

#[derive(Deserialize, Validate, Debug, DeriveIntoActiveModel)]
pub struct UserParams {
    #[validate(length(min = 1, max = 10, message = "用户名长度必须在1-10之间"))]
    pub user_code: String,
    pub username: String,
    pub password: String,
    pub dept_code: Option<String>,
    pub role_code: Option<String>,
    pub menus: Option<String>,
    // #[validate(custom(function = "crate::validation::is_mobile_phone"))]
    // pub mobile_phone:String,

    // #[serde(default)]
    // pub enabled:bool,
}

#[axum::debug_handler]
async fn create(
    State(AppState { db }): State<AppState>,
    ValidJson(params): ValidJson<UserParams>,
) -> ApiResult<ApiResponse<user::Model>> {
    let mut active_model = params.into_active_model();
    active_model.password = ActiveValue::Set(bcrypt::hash(
        // 为什么这里要&
        &active_model.password.take().unwrap(),
        bcrypt::DEFAULT_COST,
    )?);
    let result = active_model.insert(&db).await?;
    Ok(ApiResponse::success(Some(result)))
}

#[axum::debug_handler]
async fn update(
    State(AppState { db }): State<AppState>,
    // 为什么用Path,而不是ValidPath
    Path(id): Path<i32>,
    ValidJson(params): ValidJson<UserParams>,
) -> ApiResult<ApiResponse<user::Model>> {
    let existed_user = User::find_by_id(id)
        .one(&db)
        .await?
        .ok_or_else(|| ApiError::Biz(format!("User with id {} not found", id)))?;
    let password = params.password.clone();
    let mut active_model = params.into_active_model();
    active_model.id = ActiveValue::Set(existed_user.id);
    if password.is_empty() {
        // 密码为空，设置为旧密码
        active_model.password = ActiveValue::Set(existed_user.password);
    } else {
        // 密码非空，转Hash
        active_model.password = ActiveValue::Set(bcrypt::hash(&password, bcrypt::DEFAULT_COST)?);
    }
    let result = active_model.update(&db).await?;

    Ok(ApiResponse::success(Some(result)))
}

#[axum::debug_handler]
async fn delete(
    State(AppState { db }): State<AppState>,
    Path(id): Path<i32>,
) -> ApiResult<ApiResponse<()>> {
    let existed_user = User::find_by_id(id)
        .one(&db)
        .await?
        .ok_or_else(|| ApiError::Biz(format!("User with id {} not found", id)))?;
    let result = existed_user.delete(&db).await?;
    Ok(ApiResponse::success(None))
}
