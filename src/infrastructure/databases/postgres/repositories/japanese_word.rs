use anyhow::{Ok, Result};
use async_trait::async_trait;
use diesel::prelude::*;
use std::sync::Arc;

use crate::{
    domain::{
        models::japanese_word::JapaneseWord, repositories::japanese_word::JapaneseWordRepository,
    },
    infrastructure::databases::postgres::{
        models::japanese_word::JapaneseWordDiesel, postgres_connection::PgPool,
        schema::japanese_words,
    },
};

pub struct JapaneseWordDieselRepository {
    db_pool: Arc<PgPool>,
}

impl JapaneseWordDieselRepository {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl JapaneseWordRepository for JapaneseWordDieselRepository {
    async fn find_by_kanji(&self, kanji: String) -> Result<JapaneseWord> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let result = japanese_words::table
            .filter(japanese_words::kanji.eq(kanji))
            .select(JapaneseWordDiesel::as_select())
            .first::<JapaneseWordDiesel>(&mut conn)
            .map(|v| -> JapaneseWord { v.into() })?;

        Ok(result)
    }
}
