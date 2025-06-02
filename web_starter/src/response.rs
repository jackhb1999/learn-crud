use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn new(code: i32, msg: impl AsRef<str>, data: Option<T>) -> ApiResponse<T> {
        ApiResponse {
            code,
            msg: msg.as_ref().to_string(),
            data,
        }
    }
    pub fn ok(msg: impl AsRef<str>, data: Option<T>) -> Self {
        Self {
            code: 200,
            msg: msg.as_ref().to_string(),
            data,
        }
    }
    pub fn success(data: Option<T>) -> Self {
        Self {
            code: 200,
            msg: "success".to_string(),
            data,
        }
    }
    pub fn error(msg: impl AsRef<str>) -> Self {
        Self {
            code: 500,
            msg: msg.as_ref().to_string(),
            data: None,
        }
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        axum::Json(self).into_response()
    }
}
