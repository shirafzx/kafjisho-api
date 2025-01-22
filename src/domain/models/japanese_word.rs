use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JapaneseWord {
    pub id: Uuid,
    pub kanji: Option<String>,
    pub reading: Option<String>,
    pub furigana: Option<String>,
    pub createdAt: NaiveDateTime,
    pub updatedAt: NaiveDateTime,
}
