use anyhow::Result;
use futures_util::stream::TryStreamExt;
use log::info;
use sqlx::postgres::PgPoolOptions;
use sqlx::Row;
use sqlx::{Pool, Postgres};

pub async fn create_db_connection() -> Result<Pool<Postgres>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_lazy("postgres://postgres:postgres@db/GolemHub.sql")?;

    return Ok(pool);
}

pub async fn retrieve_image_data(id: u32, pool: &Pool<Postgres>) -> Result<()> {
    let mut stream = sqlx::query("SELECT * FROM public.\"Images\"").fetch(pool);

    info!("getting rows");

    while let Some(stream) = stream.try_next().await? {
        // map the row into a user-defined domain type
        let email: &str = stream.try_get("Name")?;
        info!("found image {}", email);
    }

    return Ok(());
}
