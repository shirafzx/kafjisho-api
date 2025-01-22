use anyhow::Result;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection(database_url: &str) -> Result<PgPool> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager)?;
    Ok(pool)
}
