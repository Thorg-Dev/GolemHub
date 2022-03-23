mod data_types;
mod db_connection;

use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_web::http::header::ContentType;
use sqlx::{Pool, Postgres};

use serde::Serialize;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/image/{id}")]
async fn image(db_pool: web::Data<Pool<Postgres>>, path: web::Path<u32>) -> impl Responder {
    let image = db_connection::retrieve_image_data(path.into_inner(), &db_pool)
        .await
        .expect("asd");

    let body = serde_json::to_string(&image).unwrap();

    // Create response and set content type
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(body)
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let db = db_connection::create_db_connection().await?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(hello)
            .service(image)
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
    .bind(("127.0.0.1", 8080))
    .expect("asd")
    .run()
    .await?;

    Ok(())
}