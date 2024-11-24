use sqlx::{PgPool, Postgres, Pool, Error};
use crate::configs::environment::Environment;

pub async fn connect() -> Result<Pool<Postgres>, Error> {
    let database_url: String = Environment::get_def_database(None);
    let pool: Pool<Postgres> = PgPool::connect(&database_url).await?;
    Ok(pool)
}
