use std::sync::Arc;

use anyhow::Result;

use crate::domain::{models::jp_word::JpWord, repositories::jp_word::JpWordRepository};

pub struct GetJapaneseWordUseCase<T>
where
    T: JpWordRepository + Send + Sync,
{
    japanese_word_repository: Arc<T>,
}

impl<T> GetJapaneseWordUseCase<T>
where
    T: JpWordRepository + Send + Sync,
{
    pub fn new(japanese_word_repository: Arc<T>) -> Self {
        Self {
            japanese_word_repository,
        }
    }

    pub async fn get_japanese_word(&self, kanji: String) -> Result<JpWord> {
        let result = self.japanese_word_repository.find_by_kanji(kanji).await?;

        Ok(result)
    }
}
