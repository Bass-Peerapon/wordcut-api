use std::sync::{Arc, Mutex};

use crate::application::usecase::WordcutUsecase;
use axum::{extract::Query, response::IntoResponse, Json};
use serde::Deserialize;
use serde_json::json;
#[derive(Deserialize)]
pub struct WordcutRequest {
    text: String,
}

pub async fn get_wordcut_handler(
    Query(params): Query<WordcutRequest>,
    wordcut_usecase: Arc<Mutex<WordcutUsecase>>,
) -> impl IntoResponse {
    let wordcut_usecase = wordcut_usecase.lock().unwrap();
    let result = wordcut_usecase.cut(&params.text);
    Json(json!({ "wordcut": result }))
}

pub async fn add_word_handler(
    Json(params): Json<WordcutRequest>,
    wordcut_usecase: Arc<Mutex<WordcutUsecase>>,
) -> impl IntoResponse {
    let mut wordcut_usecase = wordcut_usecase.lock().unwrap();
    wordcut_usecase.add_word(&params.text);
    Json(json!({ "result": "ok" }))
}

pub async fn remove_word_handler(
    Json(params): Json<WordcutRequest>,
    wordcut_usecase: Arc<Mutex<WordcutUsecase>>,
) -> impl IntoResponse {
    let mut wordcut_usecase = wordcut_usecase.lock().unwrap();
    wordcut_usecase.remove_word(&params.text);
    Json(json!({ "result": "ok" }))
}
