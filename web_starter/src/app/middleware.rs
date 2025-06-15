use crate::app::auth::{JWT, get_jwt};
use crate::app::error::ApiError;
use axum::body::Body;
use axum::http::{Request, Response, header};
use std::pin::Pin;
use std::sync::LazyLock;
use tower_http::auth::{AsyncAuthorizeRequest, AsyncRequireAuthorizationLayer};

#[derive(Clone)]
pub struct JWTAuth {
    jwt: &'static JWT,
}

impl JWTAuth {
    pub fn new(jwt: &'static JWT) -> Self {
        Self { jwt }
    }
}

impl AsyncAuthorizeRequest<Body> for JWTAuth {
    type RequestBody = Body;
    type ResponseBody = Body;
    type Future = Pin<
        Box<
            dyn Future<Output = Result<Request<Self::RequestBody>, Response<Self::ResponseBody>>>
                + Send
                + 'static,
        >,
    >;

    fn authorize(&mut self, mut request: Request<Body>) -> Self::Future {
        let jwt = self.jwt;
        Box::pin(async move {
            let token = request
                .headers()
                .get(header::AUTHORIZATION)
                .map(|value| -> Result<&str, ApiError> {
                    let token = value
                        .to_str()
                        .map_err(|_| {
                            ApiError::Unauthenticated(String::from(
                                "Authorization 请求头不是一个有效的字符串",
                            ))
                        })?
                        .strip_prefix("Bearer ")
                        .ok_or_else(|| {
                            ApiError::Unauthenticated(String::from(
                                "Authorization 请求头不是以 Bearer 开头的",
                            ))
                        })?;
                    Ok(token)
                })
                .transpose()?
                .ok_or_else(|| {
                    ApiError::Unauthenticated(String::from("Authorization <UNK> Bearer <UNK>"))
                })?;

            let principal = jwt
                .decode(token)
                .map_err(|err| ApiError::Internal(err.into()))?;
            request.extensions_mut().insert(principal);
            Ok(request)
        })
    }
}

static AUTH_LAYER: LazyLock<AsyncRequireAuthorizationLayer<JWTAuth>> =
    LazyLock::new(|| AsyncRequireAuthorizationLayer::<JWTAuth>::new(JWTAuth::new(get_jwt())));

pub fn get_auth_layer() -> &'static AsyncRequireAuthorizationLayer<JWTAuth> {
    &AUTH_LAYER
}
