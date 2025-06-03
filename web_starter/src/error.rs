use crate::response::ApiResponse;
use axum::extract::rejection::{JsonRejection, PathRejection, QueryRejection};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use axum_valid::{ValidRejection, ValidationRejection};

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Not Found")]
    NotFound,
    #[error("Method Not Allowed")]
    MethodNotAllowed,
    #[error("Database Error:{0}")]
    DataBase(#[from] sea_orm::DbErr),
    #[error("Internal Server Error")]
    Internal(#[from] anyhow::Error),
    #[error("0")]
    Biz(String),
    #[error("Query Error:{0}")]
    Query(#[from] QueryRejection),
    #[error("Path Error:{0}")]
    Path(#[from] PathRejection),
    #[error("Json Error:{0}")]
    Json(#[from] JsonRejection),
    #[error("Validation Error:{0}")]
    Validation(String),
    #[error("Bcrypt Error:{0}")]
    Bcrypt(#[from] bcrypt::BcryptError),
}

impl From<ValidRejection<ApiError>> for ApiError {
    fn from(value: ValidRejection<ApiError>) -> Self {
        match value {
            ValidationRejection::Valid(errors) => Self::Validation(errors.to_string()),
            ValidationRejection::Inner(e) => e,
        }
    }
}

impl ApiError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::MethodNotAllowed => StatusCode::METHOD_NOT_ALLOWED,
            Self::DataBase(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Biz(_) => StatusCode::OK,
            Self::Query(_) => StatusCode::BAD_REQUEST,
            Self::Path(_) => StatusCode::BAD_REQUEST,
            Self::Json(_) => StatusCode::BAD_REQUEST,
            Self::Validation(_) => StatusCode::BAD_REQUEST,
            Self::Bcrypt(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status_code = self.status_code();
        let body = Json(ApiResponse::<()>::error(self.to_string()));
        (status_code, body).into_response()
    }
}
