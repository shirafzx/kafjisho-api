use crate::domain::models::jp_word::JpWord;
use crate::infrastructure::databases::postgres::{models::enums::pos::Pos, schema::jp_words};

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
    pub pos: Pos,
}

impl From<JpWordDiesel> for JpWord {
    fn from(jp_word_diesel: JpWordDiesel) -> Self {
        JpWord {
            id: jp_word_diesel.id,
            kanji: jp_word_diesel.kanji,
            reading: jp_word_diesel.reading,
            furigana: jp_word_diesel.furigana,
            pos: jp_word_diesel.pos,
            created_at: jp_word_diesel.created_at,
            updated_at: jp_word_diesel.updated_at,
        }
    }
}

impl From<JpWord> for JpWordDiesel {
    fn from(jp_word: JpWord) -> Self {
        JpWordDiesel {
            id: jp_word.id,
            kanji: jp_word.kanji,
            reading: jp_word.reading,
            furigana: jp_word.furigana,
            pos: jp_word.pos,
            created_at: jp_word.created_at,
            updated_at: jp_word.updated_at,
        }
    }
}
