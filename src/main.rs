//add the modules
mod api;
mod models;
mod repository;

use std::env;

use crate::repository::mongdb_repo::db_pool;

use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use api::user::register;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json("Hello world")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = db_pool().await.unwrap();
    // init logger middleware
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .service(hello)
            .service(register)
            .app_data(web::Data::new(db.clone()))
            .wrap(middleware::Logger::default())
    })
    .bind(("localhost", 6000))?
    .run()
    .await
}
