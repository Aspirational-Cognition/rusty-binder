extern crate actix;
extern crate actix_web;
#[macro_use]
extern crate diesel;

use actix_web::{get, web, App, HttpServer, Responder};

mod actor;
mod domain;
mod schema;

#[get("/{id}/{name}/index.html")]
async fn index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
