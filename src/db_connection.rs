use crate::data_types::{Image, ProjectResponse};
use crate::{ProjectCreationRequest, ProjectModificationRequest};
use anyhow::{anyhow, Result};
use log::info;
use sqlx::postgres::PgPoolOptions;
use sqlx::QueryBuilder;
use sqlx::{Pool, Postgres};
use std::env;

use sqlx::migrate::Migrator;

static MIGRATOR: Migrator = sqlx::migrate!();

pub async fn create_db_connection(max_connections: u32) -> Result<Pool<Postgres>> {
    let username = env::var("DB_USER").unwrap_or_else(|_| String::from("postgres"));
    let password = env::var("DB_PASS").unwrap_or_else(|_| String::from("postgres"));
    let db_addr = env::var("DB_ADDR").unwrap_or_else(|_| String::from("db"));

    let url = format!(
        "postgres://{user}:{pass}@{addr}/GolemHub",
        user = username,
        pass = password,
        addr = db_addr
    );

    info!("connecting to db using url {}", url);

    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .connect_lazy(url.as_str())?;

    MIGRATOR.run(&pool).await?;

    Ok(pool)
}

pub async fn get_projects(
    offset: i32,
    limit: i32,
    pool: &Pool<Postgres>,
) -> Result<Vec<ProjectResponse>> {
    let projects = sqlx::query_as::<_, ProjectResponse>(
        r#"SELECT id, name, icon, homepage, developer, images
	    FROM public.projects
	    LIMIT $1 OFFSET $2;"#,
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(projects)
}

pub async fn add_project(
    creation_request: ProjectCreationRequest,
    pool: &Pool<Postgres>,
) -> Result<ProjectResponse> {
    let project_data = sqlx::query_as::<_, ProjectResponse>(
        r#"INSERT INTO public.projects(
        name, homepage, developer, images)
        VALUES ($1, $2, $3, $4)
        RETURNING *;"#,
    )
    .bind(creation_request.short_name)
    .bind(creation_request.homepage)
    .bind(creation_request.developer)
    .bind(
        creation_request
            .images
            .iter()
            .map(|hash| hash.to_lowercase())
            .collect::<Vec<String>>(),
    )
    .fetch_one(pool)
    .await?;

    Ok(project_data)
}

pub async fn add_icon_to_project(
    png_bytes: &[u8],
    project_id: i32,
    pool: &Pool<Postgres>,
) -> Result<()> {
    let query_result = sqlx::query(
        r#"
        UPDATE public.projects
        SET
            icon = $1
        WHERE
            id = $2
        "#,
    )
    .bind(png_bytes)
    .bind(project_id)
    .execute(pool)
    .await?;

    if query_result.rows_affected() == 0 {
        return Err(anyhow!("Failed to find project to alter"));
    }

    Ok(())
}

pub async fn delete_project(request: i32, pool: &Pool<Postgres>) -> Result<()> {
    let query_result = sqlx::query(
        r#"DELETE FROM public.projects(
        WHERE
            id=$1;"#,
    )
    .bind(request)
    .execute(pool)
    .await?;

    if query_result.rows_affected() == 0 {
        return Err(anyhow!("Failed to remove anything"));
    }

    Ok(())
}

pub async fn modify_project(
    request: ProjectModificationRequest,
    pool: &Pool<Postgres>,
) -> Result<()> {
    if request.name.is_none()
        && request.icon.is_none()
        && request.add_image.is_none()
        && request.del_image.is_none()
        && request.developer.is_none()
        && request.homepage.is_none()
    {
        return Ok(());
    }

    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE public.projects \n");

    query_builder.push("SET\n");

    if let Some(name) = request.name {
        query_builder.push("\tname = ");
        query_builder.push_bind(name);
        query_builder.push("\n");

        // let modify_query = sqlx::query(
        //     r#"
        // UPDATE public.projects
        // SET
        //     name = $1
        // WHERE
        //     id = $2
        // "#,
        // )
        // .bind(req_name)
        // .bind(request.id);
        //
        // queries.push(modify_query.execute(pool));
    }

    if let Some(request) = request.homepage {
        query_builder.push("\thomepage = ");
        query_builder.push_bind(request);
        query_builder.push("\n");
    }

    if let Some(developer) = request.developer {
        query_builder.push("\tdeveloper = ");
        query_builder.push_bind(developer);
        query_builder.push("\n");
    }

    if let Some(add_images) = request.add_image {
        for image_hash in add_images {
            query_builder.push("\timages = array_append(images, ");
            query_builder.push_bind(image_hash);
            query_builder.push(")\n");
        }
    }

    if let Some(del_images) = request.del_image {
        for image_hash in del_images {
            query_builder.push("\timages = array_remove(images, ");
            query_builder.push_bind(image_hash);
            query_builder.push(")\n");
        }
    }

    query_builder.push("WHERE id = ").push_bind(request.id);

    let query = query_builder.build();

    query.execute(pool).await?;

    Ok(())
}

pub async fn retrieve_image_by_hash(id: String, pool: &Pool<Postgres>) -> Result<Image> {
    let image_data = sqlx::query_as::<_, Image>(
        "SELECT icon
	    FROM public.projects AS projects
	    WHERE $1 = ANY(images)",
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(image_data)
}

pub async fn retrieve_project_data_by_hash(
    id: String,
    pool: &Pool<Postgres>,
) -> Result<ProjectResponse> {
    let image_data = sqlx::query_as::<_, ProjectResponse>(
        "SELECT id, name, homepage, developer, images
	    FROM public.projects AS projects
	    WHERE $1 = ANY(images)",
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(image_data)
}
