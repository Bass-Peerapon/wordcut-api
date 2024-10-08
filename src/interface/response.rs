use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub enum Response {
    Wordcut(Vec<String>),
    AddwordSuccess,
    RemovewordSuccess,
    Spellcheck(Vec<String>),
}

impl IntoResponse for Response {
    fn into_response(self) -> axum::response::Response {
        match self {
            Response::AddwordSuccess => (StatusCode::OK, Json(json!({ "message": "success" }))),
            Response::RemovewordSuccess => (StatusCode::OK, Json(json!({ "message": "success" }))),
            Response::Wordcut(words) => (StatusCode::OK, Json(json!({ "wordcut": words }))),
            Response::Spellcheck(words) => (StatusCode::OK, Json(json!({ "words": words }))),
        }
        .into_response()
    }
}
