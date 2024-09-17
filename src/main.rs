use std::sync::{Arc, Mutex};

use axum::{
    routing::{delete, post},
    Extension, Router,
};
use wordcut_api::{
    application::wordcut_usecase::WordcutUsecase,
    infrastructure::wordcut_engine::WordcutEngine,
    interface::wordcut_handler::{add_word_handler, get_wordcut_handler, remove_word_handler},
};

#[tokio::main]
async fn main() {
    let wordcut_engine = WordcutEngine::new();
    let wordcut_usecase = Arc::new(Mutex::new(WordcutUsecase::new(wordcut_engine)));

    let app = Router::new()
        .route("/wordcut", post(get_wordcut_handler))
        .route("/word", post(add_word_handler))
        .route("/word", delete(remove_word_handler))
        .layer(Extension(Arc::clone(&wordcut_usecase)));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
