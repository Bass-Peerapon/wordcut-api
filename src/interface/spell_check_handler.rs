use std::sync::{Arc, RwLock};

use crate::application::spell_cheker_usecase::SpellCheckerUsecase;
use axum::{Extension, Json};

use super::{error::Error, request::WordRequest, response::Response};

pub async fn spell_check_handler(
    Extension(spell_cheker_usecase): Extension<Arc<RwLock<SpellCheckerUsecase>>>,
    Json(params): Json<WordRequest>,
) -> Result<Response, Error> {
    let spell_checker_usecase = spell_cheker_usecase
        .read()
        .map_err(|e| Error::InternalError(e.to_string()))?;

    let result = spell_checker_usecase.check(&params.text);
    Ok(Response::Spellcheck(result))
}
