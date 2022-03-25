use anyhow::Result;
use futures_util::stream::TryStreamExt;
use log::info;

use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres, Row};

use std::env;
use crate::data_types::ProjectResponse;
use crate::ProjectCreationRequest;

pub async fn create_db_connection() -> Result<Pool<Postgres>> {

    let username = env::var("DB_USER").unwrap_or(String::from("postgres"));
    let password = env::var("DB_PASS").unwrap_or(String::from("postgres"));
    let db_addr = env::var("DB_ADDR").unwrap_or(String::from("db"));

    let url = format!("postgres://{user}:{pass}@{addr}/GolemHub", user = username, pass = password, addr = db_addr);

    info!("connecting to db using url {}", url);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_lazy(url.as_str())?;

    Ok(pool)
}

pub async fn get_projects(offset : u32, limit : u32, pool: &Pool<Postgres>) -> Result<Vec<ProjectResponse>>
{
    let projects = sqlx::query_as::<_, ProjectResponse>(r#"SELECT id, name, icon, homepage, developer, images
	    FROM public."Projects";"#).fetch_all(pool).await?;

    Ok(projects)
}

pub async fn add_project(creation_request: ProjectCreationRequest, pool: &Pool<Postgres>) -> Result<ProjectResponse> {
    let project_data = sqlx::query_as::<_, ProjectResponse>(r#"INSERT INTO public."Projects"(
        name, icon, homepage, developer, images)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *;"#)
        .bind(creation_request.short_name)
        .bind(creation_request.icon)
        .bind(creation_request.homepage)
        .bind(creation_request.developer)
        .bind(creation_request.images)
        .fetch_one(pool).await?;

    Ok(project_data)
}


pub async fn retrieve_image_data(id: u32, pool: &Pool<Postgres>) -> Result<ProjectResponse> {
    let image_data =sqlx::query_as::<_, ProjectResponse>("SELECT \"Name\", \"Id\", \"Description\", \"User\"
	    FROM public.\"Images\" AS images
	    WHERE images.\"Id\" = $1")
        .bind(id)
        .fetch_one(pool).await?;

    Ok(image_data)
}
