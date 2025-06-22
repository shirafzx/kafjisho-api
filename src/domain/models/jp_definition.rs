use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JpDefinition {
    pub id: Uuid,
    pub jp_word_id: Uuid,
    pub jp_definition: Option<String>,
    pub jp_example: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
