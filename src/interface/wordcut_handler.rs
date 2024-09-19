use std::sync::{Arc, RwLock};

use crate::application::wordcut_usecase::WordcutUsecase;
use axum::{Extension, Json};

use super::{error::Error, request::WordcutRequest, response::Response};

pub async fn get_wordcut_handler(
    Extension(wordcut_usecase): Extension<Arc<RwLock<WordcutUsecase>>>,
    Json(params): Json<WordcutRequest>,
) -> Result<Response, Error> {
    let wordcut_usecase = wordcut_usecase
        .read()
        .map_err(|e| Error::InternalError(e.to_string()))?;

    let result = wordcut_usecase.cut(&params.text);
    Ok(Response::Wordcut(result))
}
pub async fn add_word_handler(
    Extension(wordcut_usecase): Extension<Arc<RwLock<WordcutUsecase>>>,
    Json(params): Json<WordcutRequest>,
) -> Result<Response, Error> {
    let mut wordcut_usecase = wordcut_usecase
        .write()
        .map_err(|e| Error::InternalError(e.to_string()))?;

    wordcut_usecase
        .add_word(&params.text)
        .map(|_| Response::AddwordSuccess)
        .map_err(|e| Error::InternalError(e.to_string()))
}

pub async fn remove_word_handler(
    Extension(wordcut_usecase): Extension<Arc<RwLock<WordcutUsecase>>>,
    Json(params): Json<WordcutRequest>,
) -> Result<Response, Error> {
    let mut wordcut_usecase = wordcut_usecase
        .write()
        .map_err(|e| Error::InternalError(e.to_string()))?;

    wordcut_usecase
        .remove_word(&params.text)
        .map(|_| Response::RemovewordSuccess)
        .map_err(|e| Error::InternalError(e.to_string()))
}
