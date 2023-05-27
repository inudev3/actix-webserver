pub mod schema;

pub mod db_connection;
pub mod models;
pub mod handlers;
pub mod errors;
pub mod utils;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate actix_web;

use actix_identity::config::IdentityMiddlewareBuilder;
use actix_identity::IdentityMiddleware;
use actix_web::{HttpRequest, App, HttpResponse, HttpServer, web};
use actix_web::http::Method;

#[macro_use]
extern crate diesel;
extern crate dotenv;

async fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json("hello world")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let pool = db_connection::establish_connection();
    let pool_data = web::Data::new(pool);
    let csrf = CsrfMiddleware::<StdRng>::new().set_cookie(Method::GET, "/login");
    HttpServer::new(
       move || App::new()
           .wrap(csrf)
           .wrap(IdentityMiddleware::default())
            .app_data(pool_data.clone())
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


