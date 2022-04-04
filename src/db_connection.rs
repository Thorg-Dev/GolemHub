use crate::data_types::ProjectResponse;
use crate::{ProjectCreationRequest, ProjectModificationRequest};
use anyhow::{anyhow, Result};
use log::info;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;

pub async fn create_db_connection(max_connections : u32) -> Result<Pool<Postgres>> {
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

    Ok(pool)
}

pub async fn get_projects(
    offset: u32,
    limit: u32,
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
        name, icon, homepage, developer, images)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *;"#,
    )
    .bind(creation_request.short_name)
    .bind(creation_request.icon)
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
    project_id: u32,
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

pub async fn delete_project(request: u32, pool: &Pool<Postgres>) -> Result<()> {
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
    let mut queries = Vec::new();

    if request.name.is_some() {
        let modify_query = sqlx::query(
            r#"
        UPDATE public.projects
        SET
            name = $1
        WHERE
            id = $2
        "#,
        )
        .bind(request.name.unwrap())
        .bind(request.id);

        queries.push(modify_query.execute(pool));
    }

    if request.icon.is_some() {
        let modify_query = sqlx::query(
            r#"
        UPDATE public.projects
        SET
            icon = $1
        WHERE
            id = $2
        "#,
        )
        .bind(request.icon.unwrap())
        .bind(request.id);

        queries.push(modify_query.execute(pool));
    }

    if request.homepage.is_some() {
        let modify_query = sqlx::query(
            r#"
        UPDATE public.projects
        SET
            homepage = $1
        WHERE
            id = $2
        "#,
        )
        .bind(request.homepage.unwrap())
        .bind(request.id);

        queries.push(modify_query.execute(pool));
    }

    if request.developer.is_some() {
        let modify_query = sqlx::query(
            r#"
        UPDATE public.projects
        SET
            developer = $1
        WHERE
            id = $2
        "#,
        )
        .bind(request.developer.unwrap())
        .bind(request.id);

        queries.push(modify_query.execute(pool));
    }

    if request.add_image.is_some() {
        for image_hash in request.add_image.unwrap() {
            let modify_query = sqlx::query(
                r#"
                    UPDATE public.projects
                    SET
                        images = array_append(images, $1)
                    WHERE
                        id = $2
                    "#,
            )
            .bind(image_hash.to_lowercase())
            .bind(request.id);

            queries.push(modify_query.execute(pool));
        }
    }

    if request.del_image.is_some() {
        for image_hash in request.del_image.unwrap() {
            let modify_query = sqlx::query(
                r#"
                    UPDATE public.projects
                    SET
                        images = array_remove(images, $1)
                    WHERE
                        id = $2
                    "#,
            )
            .bind(image_hash.to_lowercase())
            .bind(request.id);

            queries.push(modify_query.execute(pool));
        }
    }

    for query_result in queries {
        query_result.await?;
    }

    Ok(())
}

pub async fn retrieve_project_data_by_image(
    id: String,
    pool: &Pool<Postgres>,
) -> Result<ProjectResponse> {
    let image_data = sqlx::query_as::<_, ProjectResponse>(
        "SELECT *
	    FROM public.projects AS projects
	    WHERE $1 = ANY(images)",
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(image_data)
}
