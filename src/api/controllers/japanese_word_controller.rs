use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    application::use_cases::japanese_word::{
        get_japanese_word::GetJapaneseWordUseCase,
        get_japanese_word_test::GetJapaneseWordTestUseCase,
    },
    domain::repositories::japanese_word::JapaneseWordRepository,
};

pub async fn get_japanese_word<T>(
    State(get_japanese_word_use_case): State<Arc<GetJapaneseWordUseCase<T>>>,
    Path(kanji): Path<String>,
) -> impl IntoResponse
where
    T: JapaneseWordRepository + Send + Sync,
{
    match get_japanese_word_use_case.get_japanese_word(kanji).await {
        Ok(japanese_word) => (StatusCode::OK, Json(japanese_word)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(err.to_string())).into_response(),
    }
}

pub async fn get_japanese_word_test<T>(
    State(get_japanese_word_test_use_case): State<Arc<GetJapaneseWordTestUseCase<T>>>,
    Path(kanji): Path<String>,
) -> impl IntoResponse
where
    T: JapaneseWordRepository + Send + Sync,
{
    match get_japanese_word_test_use_case
        .get_japanese_word_test(kanji)
        .await
    {
        Ok(japanese_word) => (StatusCode::OK, Json(japanese_word)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(err.to_string())).into_response(),
    }
}
