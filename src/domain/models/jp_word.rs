use crate::infrastructure::databases::postgres::models::enums::pos::Pos;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JpWord {
    pub id: Uuid,
    pub kanji: Option<String>,
    pub reading: Option<String>,
    pub furigana: Option<String>,
    pub pos: Pos,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
