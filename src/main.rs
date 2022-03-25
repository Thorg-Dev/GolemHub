mod data_types;
mod db_connection;

use anyhow::Result;

use actix_web::http::header::ContentType;
use actix_web::middleware::Logger;
use actix_web::web::Query;
use actix_web::{delete, get, patch, post, web, App, HttpResponse, HttpServer, Responder};
use sqlx::{Pool, Postgres};

use serde::Serialize;

use crate::data_types::{ProjectCreationRequest, ProjectGetQuery, ProjectResponse};

#[get("/meta/hash/{hash}")]
async fn image_hash(db_pool: web::Data<Pool<Postgres>>, path: web::Path<u32>) -> impl Responder {
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

#[patch("api/projects/{id}")]
async fn modify_project() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[delete("api/projects/{id}")]
async fn delete_project() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("api/icon")]
async fn upload_icon() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let db = db_connection::create_db_connection().await?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(hello)
            .service(image_hash)
            .service(add_project)
            .service(get_projects)
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
    .bind(("127.0.0.1", 8080))
    .expect("asd")
    .run()
    .await?;

    Ok(())
}
