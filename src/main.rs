mod data_types;
mod db_connection;


use sqlx::{Pool, Postgres};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;

//TODO GET basic routing
//TODO User schema
//TODO Image schema

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/image/{id}")]
async fn image(db_pool: web::Data<Pool<Postgres>>, path: web::Path<u32>) -> impl Responder {
    db_connection::retrieve_image_data(path.into_inner(), &db_pool).await.expect("asd");

    HttpResponse::Ok().body("response")
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

//Jaki runtime asynca?
//Jakie zmienne do pól?
//ID usera githuba?
//Download link?
//Dane do bazy w envie?
