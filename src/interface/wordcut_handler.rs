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
    let mut wordcut_usecase = wordcut_usecase.lock().unwrap();
    wordcut_usecase.add_word(&params.text);
    (StatusCode::OK, Json(json!({ "result": "ok" })))
}

pub async fn remove_word_handler(
    Extension(wordcut_usecase): Extension<Arc<Mutex<WordcutUsecase>>>,
    Json(params): Json<WordcutRequest>,
) -> impl IntoResponse {
    let mut wordcut_usecase = wordcut_usecase.lock().unwrap();
    wordcut_usecase.remove_word(&params.text);
    (StatusCode::OK, Json(json!({ "result": "ok" })))
}
