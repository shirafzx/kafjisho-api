use anyhow::Result;
use async_trait::async_trait;

use crate::domain::models::japanese_word::JapaneseWord;

#[async_trait]

pub trait JapaneseWordRepository {
    async fn find_by_kanji(&self, kanji: String) -> Result<JapaneseWord>;
}
