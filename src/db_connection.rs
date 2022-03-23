use anyhow::Result;
use futures_util::stream::TryStreamExt;
use log::info;

use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres, Row};

use std::env;
use crate::data_types::GolemImage;

pub async fn create_db_connection() -> Result<Pool<Postgres>> {

    let username = env::var("DB_USER").unwrap_or(String::from("postgres"));
    let password = env::var("DB_PASS").unwrap_or(String::from("postgres"));
    let db_addr = env::var("DB_ADDR").unwrap_or(String::from("db"));

    let url = format!("postgres://{user}:{pass}@{addr}/GolemHub", user = username, pass = password, addr = db_addr);

    info!("connecting to db using url {}", url);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_lazy(url.as_str())?;

    return Ok(pool);
}

pub async fn retrieve_image_data(id: u32, pool: &Pool<Postgres>) -> Result<GolemImage> {
    let mut image_data =sqlx::query_as::<_, GolemImage>("SELECT \"Name\", \"Id\", \"Description\", \"User\"
	    FROM public.\"Images\" AS images
	    WHERE images.\"Id\" = $1")
        .bind(id)
        .fetch_one(pool).await?;

    return Ok(image_data);
}
