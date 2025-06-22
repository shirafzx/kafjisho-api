use diesel::prelude::*;
use uuid::Uuid;

use crate::domain::models::jp_th_definition::JpThDefinition;
use crate::infrastructure::databases::postgres::schema::jp_th_definitions;

#[derive(Debug, Clone, Selectable, Queryable, Identifiable)]
#[diesel(table_name = jp_th_definitions)]
pub struct JpThDefinitionDiesel {
    pub id: Uuid,
    pub jp_def_id: Uuid,
    pub th_def_id: Uuid,
}

impl From<JpThDefinitionDiesel> for JpThDefinition {
    fn from(jp_th_definition_diesel: JpThDefinitionDiesel) -> Self {
        JpThDefinition {
            id: jp_th_definition_diesel.id,
            jp_def_id: jp_th_definition_diesel.jp_def_id,
            th_def_id: jp_th_definition_diesel.th_def_id,
        }
    }
}

impl From<JpThDefinition> for JpThDefinitionDiesel {
    fn from(jp_th_definition: JpThDefinition) -> Self {
        JpThDefinitionDiesel {
            id: jp_th_definition.id,
            jp_def_id: jp_th_definition.jp_def_id,
            th_def_id: jp_th_definition.th_def_id,
        }
    }
}
