use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JpThDefinition {
    pub id: Uuid,
    pub jp_def_id: Uuid,
    pub th_def_id: Uuid,
}
