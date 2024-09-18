use std::sync::{Arc, Mutex};

use crate::application::wordcut_usecase::WordcutUsecase;
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct WordcutRequest {
    text: String,
}

pub async fn get_wordcut_handler(
    Extension(wordcut_usecase): Extension<Arc<Mutex<WordcutUsecase>>>,
    Json(params): Json<WordcutRequest>,
) -> impl IntoResponse {
    let wordcut_usecase = wordcut_usecase.lock().unwrap();
    let result = wordcut_usecase.cut(&params.text);
    (StatusCode::OK, Json(json!({ "wordcut": result })))
}
pub async fn add_word_handler(
    Extension(wordcut_usecase): Extension<Arc<Mutex<WordcutUsecase>>>,
    Json(params): Json<WordcutRequest>,
) -> impl IntoResponse {
    let mut wordcut_usecase = match wordcut_usecase.lock() {
        Ok(wordcut_usecase) => wordcut_usecase,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Failed to lock wordcut_usecase" })),
            )
        }
    };

    match wordcut_usecase.add_word(&params.text) {
        Ok(_) => (StatusCode::OK, Json(json!({ "result": "ok" }))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        ),
    }
}

pub async fn remove_word_handler(
    Extension(wordcut_usecase): Extension<Arc<Mutex<WordcutUsecase>>>,
    Json(params): Json<WordcutRequest>,
) -> impl IntoResponse {
    let mut wordcut_usecase = match wordcut_usecase.lock() {
        Ok(wordcut_usecase) => wordcut_usecase,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Failed to lock wordcut_usecase" })),
            )
        }
    };

    match wordcut_usecase.remove_word(&params.text) {
        Ok(_) => (StatusCode::OK, Json(json!({ "result": "ok" }))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        ),
    }
}
