use std::sync::{Arc, Mutex};

use axum::{
    routing::{delete, get, post},
    Extension, Router,
};
use wordcut_api::{
    application::wordcut::wordcut_usecase,
    infrastructure::wordcut_engine,
    interface::handler::wordcut_handler::{
        add_word_handler, get_wordcut_handler, remove_word_handler,
    },
};

#[tokio::main]
async fn main() {
    let wordcut_engine = wordcut_engine::new();
    let wordcut_usecase = Arc::new(Mutex::new(wordcut_usecase::new(wordcut_engine)));

    let app = Router::new()
        .route(
            "/wordcut",
            get(get_wordcut_handler::<
                wordcut_usecase::WordcutApplication<wordcut_engine::WordcutEngineSdk>,
            >),
        )
        .route(
            "/wordcut",
            post(
                add_word_handler::<
                    wordcut_usecase::WordcutApplication<wordcut_engine::WordcutEngineSdk>,
                >,
            ),
        )
        .route(
            "/wordcut",
            delete(
                remove_word_handler::<
                    wordcut_usecase::WordcutApplication<wordcut_engine::WordcutEngineSdk>,
                >,
            ),
        )
        .layer(Extension(Arc::clone(&wordcut_usecase)));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
