use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub enum Error {
    InternalError(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::InternalError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e })),
            ),
        }
        .into_response()
    }
}
