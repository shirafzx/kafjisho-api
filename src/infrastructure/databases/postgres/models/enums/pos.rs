use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Debug, Clone, Deserialize, Serialize, AsExpression, FromSqlRow)]
#[diesel(sql_type = crate::infrastructure::databases::postgres::schema::sql_types::Pos)]
pub enum Pos {
    NOUN,
    PRONOUN,
    VERB,
    ADVERB,
    ADJECTIVE,
    PREPOSITION,
    CONJUNCTION,
    INTERJECTION,
}

impl ToSql<crate::infrastructure::databases::postgres::schema::sql_types::Pos, Pg> for Pos {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            Pos::NOUN => out.write_all(b"NOUN")?,
            Pos::PRONOUN => out.write_all(b"PRONOUN")?,
            Pos::VERB => out.write_all(b"VERB")?,
            Pos::ADVERB => out.write_all(b"ADVERB")?,
            Pos::ADJECTIVE => out.write_all(b"ADJECTIVE")?,
            Pos::PREPOSITION => out.write_all(b"PREPOSITION")?,
            Pos::CONJUNCTION => out.write_all(b"CONJUNCTION")?,
            Pos::INTERJECTION => out.write_all(b"INTERJECTION")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::infrastructure::databases::postgres::schema::sql_types::Pos, Pg> for Pos {
    fn from_sql(bytes: diesel::pg::PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"NOUN" => Ok(Pos::NOUN),
            b"PRONOUN" => Ok(Pos::PRONOUN),
            b"VERB" => Ok(Pos::VERB),
            b"ADVERB" => Ok(Pos::ADVERB),
            b"ADJECTIVE" => Ok(Pos::ADJECTIVE),
            b"PREPOSITION" => Ok(Pos::PREPOSITION),
            b"CONJUNCTION" => Ok(Pos::CONJUNCTION),
            b"INTERJECTION" => Ok(Pos::INTERJECTION),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
