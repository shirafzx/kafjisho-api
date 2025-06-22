use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

use crate::infrastructure::databases::postgres::schema::th_words;

use super::enums::pos::Pos;
use crate::domain::models::th_word::ThWord;

#[derive(Debug, Clone, Selectable, Queryable, Identifiable)]
#[diesel(table_name = th_words)]
pub struct ThWordDiesel {
    pub id: Uuid,
    pub word: Option<String>,
    pub pos: Pos,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<ThWordDiesel> for ThWord {
    fn from(th_word_diesel: ThWordDiesel) -> Self {
        ThWord {
            id: th_word_diesel.id,
            word: th_word_diesel.word,
            pos: th_word_diesel.pos,
            created_at: th_word_diesel.created_at,
            updated_at: th_word_diesel.updated_at,
        }
    }
}

impl From<ThWord> for ThWordDiesel {
    fn from(th_word: ThWord) -> Self {
        ThWordDiesel {
            id: th_word.id,
            word: th_word.word,
            pos: th_word.pos,
            created_at: th_word.created_at,
            updated_at: th_word.updated_at,
        }
    }
}
