mod data_types;
mod db_connection;

use crate::data_types::{ProjectCreationRequest, ProjectGetQuery, ProjectModificationRequest};
use actix_multipart::Multipart;
use actix_web::middleware::Logger;
use actix_web::web::{Bytes, Query};
use actix_web::{delete, get, patch, post, web, App, HttpResponse, HttpServer, Responder};
use futures_util::{StreamExt, TryStreamExt};
use image::io::Reader as ImageReader;
use sqlx::{Pool, Postgres};
use std::io::Cursor;

#[get("/meta/hash/{hash}")]
async fn get_image(db_pool: web::Data<Pool<Postgres>>, path: web::Path<u32>) -> impl Responder {
    let project_response = db_connection::retrieve_image_data(path.into_inner(), &db_pool).await;

    if project_response.is_err() {
        return HttpResponse::Accepted().body("Image not found");
    }

    HttpResponse::Ok().json(project_response.unwrap())
}

#[get("/api/projects")]
async fn get_projects(
    db_pool: web::Data<Pool<Postgres>>,
    info: Query<ProjectGetQuery>,
) -> impl Responder {
    let found_projects = db_connection::get_projects(info.offset, info.limit, &db_pool).await;

    match found_projects {
        Err(e) => HttpResponse::Accepted().body(e.to_string()),
        Ok(project_val) => HttpResponse::Ok().json(project_val),
    }
}

#[post("api/projects")]
async fn add_project(
    db_pool: web::Data<Pool<Postgres>>,
    request: web::Json<ProjectCreationRequest>,
) -> impl Responder {
    //TODO validate request?

    let added_project = db_connection::add_project(request.into_inner(), &db_pool).await;

    match added_project {
        Err(e) => HttpResponse::Accepted().body(e.to_string()),
        Ok(project_val) => HttpResponse::Ok().body(serde_json::to_string(&project_val).unwrap()),
    }
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
    path: web::Path<u32>,
    db_pool: web::Data<Pool<Postgres>>,
) -> impl Responder {
    let result = db_connection::delete_project(path.into_inner(), &db_pool).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Delete succesful"),
        Err(e) => HttpResponse::Accepted().body(e.to_string()),
    }
}

#[post("api/icon/{id}")]
async fn upload_icon(
    path: web::Path<u32>,
    mut payload: Multipart,
    db_pool: web::Data<Pool<Postgres>>,
) -> impl Responder {
    let project_id = path.into_inner();

    while let Ok(Some(mut field)) = payload.try_next().await {
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            let data2 = Bytes::copy_from_slice(data.as_ref());
            let image_reader = ImageReader::new(Cursor::new(data2));

            let potential_image = image_reader.with_guessed_format();

            let image = match potential_image {
                Ok(format) => format,
                Err(e) => return HttpResponse::Ok().body(e.to_string()),
            };

            if image.format().is_none() {
                return HttpResponse::Accepted().body("Provided file is not png");
            }
            let result =
                db_connection::add_icon_to_project(data.as_ref(), project_id, &db_pool).await;

            match result {
                Ok(_) => return HttpResponse::Ok()
                    .body(format!("Icon uploaded to project with id {}", project_id)),
                Err(e) => return HttpResponse::Accepted().body(e.to_string()),
            }
        }
    }
    HttpResponse::Accepted().body("Failed to upload icon")
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let db = db_connection::create_db_connection().await?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(get_image)
            .service(add_project)
            .service(get_projects)
            .service(modify_project)
            .service(delete_project)
            .service(upload_icon)
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
    .bind(("127.0.0.1", 8080))
    .expect("asd")
    .run()
    .await?;

    Ok(())
}
