use crate::app::error::ApiError;
use crate::app::path::Path;
use crate::app::query::Query;
use axum::extract::{FromRequest, FromRequestParts, Request};
use axum::http::request::Parts;
use crate::app::json::Json;

#[derive(Debug, Clone, Default, FromRequest, FromRequestParts)]
#[from_request(via(axum_valid::Valid), rejection(ApiError))]
pub struct Valid<T>(pub T);

#[derive(Debug, Clone, Default)]
pub struct ValidQuery<T>(pub T);
#[derive(Debug, Clone, Default)]
pub struct ValidPath<T>(pub T);
#[derive(Debug, Clone, Default)]
pub struct ValidJson<T>(pub T);

// 原始实现
// impl<S, T> FromRequestParts<S> for ValidQuery<T>
// where
//     S: Send + Sync,
//     Query<T>: FromRequestParts<S, Rejection = ApiError> + axum_valid::HasValidate,
//     <Query<T> as axum_valid::HasValidate>::Validate: validator::Validate,
// {
//     type Rejection = ApiError;
//
//     async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
//         let result = axum_valid::Valid::<Query<T>>::from_request_parts(parts, state).await?;
//         Ok(ValidQuery(result.0.0))
//     }
// }

// 借助已有的 Valid 实现
// impl<S, T> FromRequestParts<S> for ValidQuery<T>
// where
//     S: Send + Sync,
//     Valid<Query<T>>: FromRequestParts<S, Rejection = ApiError>,
// {
//     type Rejection = ApiError;
//
//     async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
//         Ok(ValidQuery(
//             Valid::<Query<T>>::from_request_parts(parts, state)
//                 .await?
//                 .0
//                 .0,
//         ))
//     }
// }

// impl<S, T> FromRequest<S> for ValidJson<T>{
//     type Rejection = ();
// 
//     fn from_request(req: Request, state: &S) -> impl Future<Output=Result<Self, Self::Rejection>> + Send {
//         todo!()
//     }
// }

macro_rules! impl_from_request {
    ($name:ident,$wrapper:ident,FromRequestParts) => {
        impl<S, T> FromRequestParts<S> for $name<T>
        where
            S: Send + Sync,
            Valid<$wrapper<T>>: FromRequestParts<S, Rejection = ApiError>,
        {
            type Rejection = ApiError;

            async fn from_request_parts(
                parts: &mut Parts,
                state: &S,
            ) -> Result<Self, Self::Rejection> {
                Ok($name(
                    Valid::<$wrapper<T>>::from_request_parts(parts, state)
                        .await?
                        .0
                        .0,
                ))
            }
        }
    };
    ($name:ident,$wrapper:ident,FromRequest) => {
        impl<S, T> FromRequest<S> for $name<T>
        where
            S: Send + Sync,
            Valid<$wrapper<T>>: FromRequest<S, Rejection = ApiError>,
        {
            type Rejection = ApiError;

            async fn from_request(    
                req: Request,
        state: &S,
            ) -> Result<Self, Self::Rejection> {
                Ok($name(
                    Valid::<$wrapper<T>>::from_request(req, state)
                        .await?
                        .0
                        .0,
                ))
            }
        }
    };
}
impl_from_request!(ValidQuery, Query, FromRequestParts);
impl_from_request!(ValidPath, Path, FromRequestParts);
impl_from_request!(ValidJson, Json, FromRequest);
