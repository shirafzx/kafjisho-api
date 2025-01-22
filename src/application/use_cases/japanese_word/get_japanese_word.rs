use std::sync::Arc;

use anyhow::Result;

use crate::domain::{
    models::japanese_word::JapaneseWord, repositories::japanese_word::JapaneseWordRepository,
};

pub struct GetJapaneseWordUseCase<T>
where
    T: JapaneseWordRepository + Send + Sync,
{
    japanese_word_repository: Arc<T>,
}

impl<T> GetJapaneseWordUseCase<T>
where
    T: JapaneseWordRepository + Send + Sync,
{
    pub fn new(japanese_word_repository: Arc<T>) -> Self {
        Self {
            japanese_word_repository,
        }
    }

    pub async fn get_japanese_word(&self, kanji: String) -> Result<JapaneseWord> {
        let result = self.japanese_word_repository.find_by_kanji(kanji).await?;

        Ok(result)
    }
}
