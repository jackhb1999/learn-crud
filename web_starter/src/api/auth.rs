use crate::app::AppState;
use crate::app::auth::{Principal, get_jwt};
use crate::app::error::{ApiError, ApiResult};
use crate::app::middleware::get_auth_layer;
use crate::app::response::ApiResponse;
use crate::app::utils::verify_password;
use crate::app::valid::ValidJson;
use crate::entity::prelude::*;
use crate::entity::user;
use axum::extract::{ConnectInfo, State};
use axum::{Extension, Router, debug_handler, routing};
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use validator::Validate;

pub fn create_router() -> Router<AppState> {
    // 自下而上匹配
    Router::new()
        .route("/user-info", routing::get(get_user_info))
        .route_layer(get_auth_layer())
        .route("/login", routing::post(login))
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginParams {
    #[validate(length(min = 3, max = 10, message = "账号的长度为3-10"))]
    account: String,
    #[validate(length(min = 3, max = 10, message = "密码的3-10"))]
    password: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResult {
    access_token: String,
}

#[debug_handler]
#[tracing::instrument(name = "Login", skip_all,fields(account=%params.account,ip=%addr))]
async fn login(
    State(AppState { db }): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    ValidJson(params): ValidJson<LoginParams>,
) -> ApiResult<ApiResponse<LoginResult>> {
    tracing::info!("开始处理：登录");
    let user = User::find()
        .filter(user::Column::UserCode.eq(&params.account))
        .one(&db)
        .await?
        .ok_or_else(|| ApiError::Biz("账号不存在".to_string()))?;
    if !verify_password(&params.password, &user.password)? {
        return Err(ApiError::Biz(String::from("密码错误")));
    }
    let principal = Principal {
        id: user.id.to_string(),
        name: user.username,
    };
    let access_token = get_jwt().encode(principal)?;
    tracing::info!("登录成功");
    Ok(ApiResponse::ok(
        "登录成功",
        Some(LoginResult { access_token }),
    ))
}

async fn get_user_info(
    Extension(principal): Extension<Principal>,
) -> ApiResult<ApiResponse<Principal>> {
    Ok(ApiResponse::ok("登录成功", Some(principal)))
}
