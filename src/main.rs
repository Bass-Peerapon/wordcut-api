use std::sync::{Arc, Mutex};

use axum::{
    routing::{delete, get, post},
    Router,
};
use wordcut_api::{
    application::usecase::WordcutUsecase,
    infrastructure::web::{add_word_handler, get_wordcut_handler, remove_word_handler},
};

#[tokio::main]
async fn main() {
    let wordcut_usecase = Arc::new(Mutex::new(WordcutUsecase::new()));

    let app = Router::new()
        .route(
            "/wordcut",
            get({
                let wordcut_usecase = Arc::clone(&wordcut_usecase);
                move |req| get_wordcut_handler(req, wordcut_usecase)
            }),
        )
        .route(
            "/wordcut",
            post({
                let wordcut_usecase = Arc::clone(&wordcut_usecase);
                move |req| add_word_handler(req, wordcut_usecase)
            }),
        )
        .route(
            "/wordcut",
            delete({
                let wordcut_usecase = Arc::clone(&wordcut_usecase);
                move |req| remove_word_handler(req, wordcut_usecase)
            }),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
