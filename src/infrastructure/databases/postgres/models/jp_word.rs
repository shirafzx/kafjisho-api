use crate::{
    domain::models::jp_word::JpWord, infrastructure::databases::postgres::schema::jp_words,
};

use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Debug, Clone, Selectable, Queryable, Identifiable)]
#[diesel(table_name = jp_words)]
pub struct JpWordDiesel {
    pub id: Uuid,
    pub kanji: Option<String>,
    pub reading: Option<String>,
    pub furigana: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<JpWordDiesel> for JpWord {
    fn from(japanese_word_diesel: JpWordDiesel) -> Self {
        JpWord {
            id: japanese_word_diesel.id,
            kanji: japanese_word_diesel.kanji,
            reading: japanese_word_diesel.reading,
            furigana: japanese_word_diesel.furigana,
            createdAt: japanese_word_diesel.created_at,
            updatedAt: japanese_word_diesel.updated_at,
        }
    }
}

impl From<JpWord> for JpWordDiesel {
    fn from(japanese_word: JpWord) -> Self {
        JpWordDiesel {
            id: japanese_word.id,
            kanji: japanese_word.kanji,
            reading: japanese_word.reading,
            furigana: japanese_word.furigana,
            created_at: japanese_word.createdAt,
            updated_at: japanese_word.updatedAt,
        }
    }
}
