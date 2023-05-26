pub mod schema;

pub mod db_connection;
pub mod models;
pub mod handlers;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate actix_web;

use actix_web::{HttpRequest, App, HttpResponse, HttpServer, web};

#[macro_use]
extern crate diesel;
extern crate dotenv;

async fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json("hello world")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(
        || App::new()
            .service(
                web::resource("/products")
                    .route(web::get().to(handlers::products::index))
                    .route(web::post().to(handlers::products::create))
            )
            .service(
                web::resource("/products/{id}")
                    .route(web::get().to(handlers::products::show))
                    .route(web::delete().to(handlers::products::destroy))
                    .route(web::patch().to(handlers::products::update))
            )
    )
        .bind("127.0.0.1:8080")?.run().await
    // println!("Started http server: 127.0.0.1:8080");
}


