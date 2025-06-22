use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThDefinition {
    pub id: Uuid,
    pub th_word_id: Uuid,
    pub th_definition: Option<String>,
    pub th_example: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
