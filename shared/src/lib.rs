use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn get_db_pool() -> anyhow::Result<Pool<Postgres>> {
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://swapi_sauvignon:password@localhost/swapi_sauvignon")
        .await?;

    Ok(db_pool)
}
