use crate::infrastructure::databases::postgres::models::enums::pos::Pos;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThWord {
    pub id: Uuid,
    pub word: Option<String>,
    pub pos: Pos,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
