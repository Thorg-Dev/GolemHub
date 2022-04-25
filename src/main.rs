mod data_types;
mod db_connection;

use crate::data_types::{
    ProjectCreationRequest, ProjectGetQuery, ProjectModificationRequest, ProjectResponse,
};
use actix_multipart::Multipart;
use actix_web::middleware::Logger;
use actix_web::web::{Bytes, Json, Query};
use actix_web::{delete, get, patch, post, web, App, HttpResponse, HttpServer, Responder};
use futures_util::{StreamExt, TryStreamExt};
use image::io::Reader as ImageReader;
use sqlx::{Pool, Postgres};
use std::io::Cursor;

#[get("/meta/hash/{hash}")]
async fn get_image(
    db_pool: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
) -> actix_web::Result<Json<ProjectResponse>> {
    let project_response =
        db_connection::retrieve_project_data_by_hash(path.into_inner().to_lowercase(), &db_pool)
            .await
            .map_err(|e| actix_web::error::ErrorNotFound(e))?;

    Ok(web::Json(project_response))
}

#[get("/api/projects")]
async fn get_projects(
    db_pool: web::Data<Pool<Postgres>>,
    info: Query<ProjectGetQuery>,
) -> actix_web::Result<Json<Vec<ProjectResponse>>> {
    let found_projects = db_connection::get_projects(info.offset, info.limit, &db_pool)
        .await
        .map_err(|e| actix_web::error::ErrorNotFound(e))?;

    Ok(Json(found_projects))
}

#[post("api/projects")]
async fn add_project(
    db_pool: web::Data<Pool<Postgres>>,
    request: web::Json<ProjectCreationRequest>,
) -> actix_web::Result<Json<ProjectResponse>> {
    let added_project = db_connection::add_project(request.into_inner(), &db_pool)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(Json(added_project))
}

#[patch("api/projects")]
async fn modify_project(
    db_pool: web::Data<Pool<Postgres>>,
    request: web::Json<ProjectModificationRequest>,
) -> impl Responder {
    let result = db_connection::modify_project(request.into_inner(), &db_pool).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Project modified"),
        Err(_) => HttpResponse::Accepted().body("failed to modify project"),
    }
}

#[delete("api/projects/{id}")]
async fn delete_project(
    path: web::Path<i32>,
    db_pool: web::Data<Pool<Postgres>>,
) -> impl Responder {
    let result = db_connection::delete_project(path.into_inner(), &db_pool).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Delete succesful"),
        Err(e) => HttpResponse::Accepted().body(e.to_string()),
    }
}

#[get("api/icon/{hash}")]
async fn get_icon_by_hash(
    path: web::Path<String>,
    db_pool: web::Data<Pool<Postgres>>,
) -> actix_web::Result<actix_web::web::Bytes> {
    let image_bytes_result = db_connection::retrieve_image_by_hash(path.into_inner(), &db_pool)
        .await
        .map_err(|e| actix_web::error::ErrorNotFound(e))?;

    Ok(Bytes::from(image_bytes_result.icon))
}

#[post("api/icon/{id}")]
async fn upload_icon(
    path: web::Path<i32>,
    mut payload: Multipart,
    db_pool: web::Data<Pool<Postgres>>,
) -> impl Responder {
    let project_id = path.into_inner();

    while let Ok(Some(mut field)) = payload.try_next().await {
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            let image_reader = ImageReader::new(Cursor::new(&data));

            let potential_image = image_reader.with_guessed_format();

            let image = match potential_image {
                Ok(format) => format,
                Err(e) => return HttpResponse::Ok().body(e.to_string()),
            };

            if image.format().is_none() {
                return HttpResponse::Accepted().body("Provided file is not png");
            }
            let result = db_connection::add_icon_to_project(&data, project_id, &db_pool).await;

            match result {
                Ok(_) => {
                    return HttpResponse::Ok()
                        .body(format!("Icon uploaded to project with id {}", project_id))
                }
                Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
            }
        }
    }
    HttpResponse::BadRequest().body("Failed to upload icon")
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let db = db_connection::create_db_connection(5).await?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(get_image)
            .service(get_projects)
            .service(get_icon_by_hash)
            // TODO reenable non-get services (requires authorization layer)
            // .service(add_project)
            // .service(modify_project)
            // .service(delete_project)
            // .service(upload_icon)
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
    .bind(("0.0.0.0", 8080))
    .expect("Failed to start server")
    .run()
    .await?;

    Ok(())
}

//WORKS ONLY VIA docker-compose.test.yml
#[cfg(test)]
mod tests {
    use crate::db_connection::create_db_connection;
    use actix_web::http::StatusCode;
    use actix_web::{test, App};

    use super::*;

    #[actix_web::test]
    async fn test_index_get() {
        env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

        let pool = create_db_connection(1)
            .await
            .expect("failed to create pool");

        sqlx::query("BEGIN")
            .execute(&pool)
            .await
            .expect("BEGIN failed");

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(get_projects)
                .wrap(Logger::new("%a %{User-Agent}i")),
        )
        .await;
        let req = test::TestRequest::get().uri("/api/projects").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }
}
