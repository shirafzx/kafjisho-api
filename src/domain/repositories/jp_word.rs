use anyhow::Result;
use async_trait::async_trait;

use crate::domain::models::jp_word::JpWord;

#[async_trait]

pub trait JpWordRepository {
    async fn find_by_kanji(&self, kanji: String) -> Result<JpWord>;
}
