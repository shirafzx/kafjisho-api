use std::sync::Arc;

use axum::{routing::get, Router};

use crate::{
    api::controllers::japanese_word_controller::get_japanese_word,
    application::use_cases::japanese_word::get_japanese_word::GetJapaneseWordUseCase,
    infrastructure::databases::postgres::{
        postgres_connection::PgPool, repositories::japanese_word::JapaneseWordDieselRepository,
    },
};

pub fn routes(db_pool: Arc<PgPool>) -> Router {
    let japanese_word_repository = JapaneseWordDieselRepository::new(db_pool);
    let get_japanese_word_use_case =
        GetJapaneseWordUseCase::new(Arc::new(japanese_word_repository));

    Router::new()
        .route("/{kanji}", get(get_japanese_word))
        .with_state(Arc::new(get_japanese_word_use_case))
}
