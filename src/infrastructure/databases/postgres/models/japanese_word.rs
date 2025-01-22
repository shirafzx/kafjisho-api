use crate::{
    domain::models::japanese_word::JapaneseWord,
    infrastructure::databases::postgres::schema::japanese_words,
};

use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Debug, Clone, Selectable, Queryable, Identifiable)]
#[diesel(table_name = japanese_words)]
pub struct JapaneseWordDiesel {
    pub id: Uuid,
    pub kanji: Option<String>,
    pub reading: Option<String>,
    pub furigana: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<JapaneseWordDiesel> for JapaneseWord {
    fn from(japanese_word_diesel: JapaneseWordDiesel) -> Self {
        JapaneseWord {
            id: japanese_word_diesel.id,
            kanji: japanese_word_diesel.kanji,
            reading: japanese_word_diesel.reading,
            furigana: japanese_word_diesel.furigana,
            createdAt: japanese_word_diesel.created_at,
            updatedAt: japanese_word_diesel.updated_at,
        }
    }
}

impl From<JapaneseWord> for JapaneseWordDiesel {
    fn from(japanese_word: JapaneseWord) -> Self {
        JapaneseWordDiesel {
            id: japanese_word.id,
            kanji: japanese_word.kanji,
            reading: japanese_word.reading,
            furigana: japanese_word.furigana,
            created_at: japanese_word.createdAt,
            updated_at: japanese_word.updatedAt,
        }
    }
}
