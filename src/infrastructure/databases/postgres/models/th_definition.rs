use crate::domain::models::th_definition::ThDefinition;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

use crate::infrastructure::databases::postgres::schema::th_definitions;

#[derive(Debug, Clone, Selectable, Queryable, Identifiable)]
#[diesel(table_name = th_definitions)]
pub struct ThDefinitionDiesel {
    pub id: Uuid,
    pub th_word_id: Uuid,
    pub th_definition: Option<String>,
    pub th_example: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<ThDefinitionDiesel> for ThDefinition {
    fn from(th_definition_diesel: ThDefinitionDiesel) -> Self {
        ThDefinition {
            id: th_definition_diesel.id,
            th_word_id: th_definition_diesel.th_word_id,
            th_definition: th_definition_diesel.th_definition,
            th_example: th_definition_diesel.th_example,
            created_at: th_definition_diesel.created_at,
            updated_at: th_definition_diesel.updated_at,
        }
    }
}

impl From<ThDefinition> for ThDefinitionDiesel {
    fn from(th_definition: ThDefinition) -> Self {
        ThDefinitionDiesel {
            id: th_definition.id,
            th_word_id: th_definition.th_word_id,
            th_definition: th_definition.th_definition,
            th_example: th_definition.th_example,
            created_at: th_definition.created_at,
            updated_at: th_definition.updated_at,
        }
    }
}
