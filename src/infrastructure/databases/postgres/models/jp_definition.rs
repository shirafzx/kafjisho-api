use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

use crate::domain::models::jp_definition::JpDefinition;
use crate::infrastructure::databases::postgres::schema::jp_definitions;

#[derive(Debug, Clone, Selectable, Queryable, Identifiable)]
#[diesel(table_name = jp_definitions)]
pub struct JpDefinitionDiesel {
    pub id: Uuid,
    pub jp_word_id: Uuid,
    pub jp_definition: Option<String>,
    pub jp_example: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<JpDefinitionDiesel> for JpDefinition {
    fn from(jp_definition_diesel: JpDefinitionDiesel) -> Self {
        JpDefinition {
            id: jp_definition_diesel.id,
            jp_word_id: jp_definition_diesel.jp_word_id,
            jp_definition: jp_definition_diesel.jp_definition,
            jp_example: jp_definition_diesel.jp_example,
            created_at: jp_definition_diesel.created_at,
            updated_at: jp_definition_diesel.updated_at,
        }
    }
}

impl From<JpDefinition> for JpDefinitionDiesel {
    fn from(jp_definition: JpDefinition) -> Self {
        JpDefinitionDiesel {
            id: jp_definition.id,
            jp_word_id: jp_definition.jp_word_id,
            jp_definition: jp_definition.jp_definition,
            jp_example: jp_definition.jp_example,
            created_at: jp_definition.created_at,
            updated_at: jp_definition.updated_at,
        }
    }
}
