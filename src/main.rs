use std::sync::{Arc, RwLock};

use axum::{
    routing::{delete, post},
    Extension, Router,
};
use wordcut_api::{
    application::{spell_cheker_usecase::SpellCheckerUsecase, wordcut_usecase::WordcutUsecase},
    infrastructure::{spell_checker::SpellChecker, wordcut_engine::WordcutEngine},
    interface::{
        spell_check_handler::spell_check_handler,
        wordcut_handler::{add_word_handler, get_wordcut_handler, remove_word_handler},
    },
};

#[tokio::main]
async fn main() {
    let spell_checker = SpellChecker::new();
    let wordcut_engine = WordcutEngine::new().unwrap();

    let wordcut_usecase = Arc::new(RwLock::new(WordcutUsecase::new(wordcut_engine)));
    let spell_checker_usecase = Arc::new(RwLock::new(SpellCheckerUsecase::new(spell_checker)));
    let app = Router::new()
        .route("/wordcut", post(get_wordcut_handler))
        .route("/word", post(add_word_handler))
        .route("/word", delete(remove_word_handler))
        .route("/spellcheck", post(spell_check_handler))
        .layer(Extension(Arc::clone(&wordcut_usecase)))
        .layer(Extension(Arc::clone(&spell_checker_usecase)));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
