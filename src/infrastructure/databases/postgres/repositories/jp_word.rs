use anyhow::{Ok, Result};
use async_trait::async_trait;
use diesel::prelude::*;
use std::sync::Arc;

use crate::{
    domain::{models::jp_word::JpWord, repositories::jp_word::JpWordRepository},
    infrastructure::databases::postgres::{
        models::jp_word::JpWordDiesel, postgres_connection::PgPool, schema::jp_words,
    },
};

pub struct JpWordDieselRepository {
    db_pool: Arc<PgPool>,
}

impl JpWordDieselRepository {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        Self { db_pool }
    }
}

impl Clone for JpWordDieselRepository {
    fn clone(&self) -> Self {
        Self {
            db_pool: Arc::clone(&self.db_pool),
        }
    }
}

#[async_trait]
impl JpWordRepository for JpWordDieselRepository {
    async fn find_by_kanji(&self, kanji: String) -> Result<JpWord> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let result = jp_words::table
            .filter(jp_words::kanji.eq(kanji))
            .select(JpWordDiesel::as_select())
            .first::<JpWordDiesel>(&mut conn)
            .map(|v| -> JpWord { v.into() })?;

        Ok(result)
    }
}
