mod data_types;
mod db_connection;

use crate::data_types::{ProjectCreationRequest, ProjectGetQuery, ProjectModificationRequest};
use actix_multipart::Multipart;
use actix_web::middleware::Logger;
use actix_web::web::{Buf, Bytes, Query};
use actix_web::{delete, get, patch, post, web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use futures_util::{StreamExt, TryStreamExt};
use image::codecs::png;
use image::io::Reader as ImageReader;
use log::info;
use serde::Serialize;
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

    HttpResponse::Ok().body("Project modified")
}

#[delete("api/projects/{id}")]
async fn delete_project() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("api/icon/{id}")]
async fn upload_icon(
    path: web::Path<u32>,
    mut payload: Multipart,
    db_pool: web::Data<Pool<Postgres>>,
) -> impl Responder {
    let project_id = path.into_inner();

    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition();
        let filename = content_type.get_filename().unwrap();
        let filepath = format!(".{}", filename);
        //info!("Found file {}", filepath);

        // // File::create is blocking operation, use threadpool
        // let mut f = web::block(|| std::fs::File::create(filepath))
        //     .await
        //     .unwrap();

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            let data2 = Bytes::copy_from_slice(data.as_ref());

            // TODO reenable png check
            // let image_reader = ImageReader::new(Cursor::new(data));
            //
            // let potential_image = image_reader.with_guessed_format();
            //
            // let image = match potential_image {
            //     Ok(format) => format,
            //     Err(e) => return HttpResponse::Ok().body(e.to_string()),
            // };

            //.decode().unwrap();

            db_connection::add_icon_to_project(data.as_ref(), project_id, &db_pool).await;

            //info!("{:?}", potential_image);

            // // filesystem operations are blocking, we have to use threadpool
            // f = web::block(move || f.write_all(&data).map(|_| f))
            //     .await
            //     .unwrap();
        }
    }
    HttpResponse::Ok().body("Hello world!")
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
            .service(upload_icon)
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
    .bind(("127.0.0.1", 8080))
    .expect("asd")
    .run()
    .await?;

    Ok(())
}
