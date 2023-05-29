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


use actix_cors::Cors;
use actix_identity::{CookieIdentityPolicy,  IdentityService};
use actix_web::{HttpRequest, App, HttpResponse, HttpServer, web};
use actix_web::cookie::time;
use actix_web::http::{header, Method};
use actix_web::middleware::{ Logger};
use csrf::{AesGcmCsrfProtection, CsrfProtection};

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
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


    let csrf_token_header = header::HeaderName::from_lowercase(b"x-csrf-token").unwrap();
    HttpServer::new(
       move || App::new()
           .app_data(AesGcmCsrfProtection::from_key(dotenv!("CSRF_SECRET_KEY").as_bytes().try_into().expect("CSRF KEY not sized appropriately")))
           .wrap(IdentityService::new(
               CookieIdentityPolicy::new(dotenv!("SECRET_KEY").as_bytes())
                   .name("mystorejwt")
                   .path("/")
                   .domain(dotenv!("MYSTOREDOMAIN"))
                   .max_age(time::Duration::days(1))
                   .secure(dotenv!("COOKIE_SECURE").parse().unwrap())
           ))
           .wrap(
                    Cors::default()
                   .allowed_origin(dotenv!("ALLOWED_ORIGIN"))
                   .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                   .allowed_headers(vec![header::AUTHORIZATION,
                                         header::CONTENT_TYPE,
                                         header::ACCEPT,
                                         csrf_token_header.clone()])
                   .expose_headers(vec![csrf_token_header.clone()])
                   .max_age(3600)
           )
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
           .service(
               web::resource("/register")
                   .route(web::post().to(handlers::register::register))
           )
           .service(
               web::resource("/auth")
                   .route(web::post().to(handlers::authentication::login))
                   .route(web::delete().to(handlers::authentication::logout))
           )
    )
        .bind("127.0.0.1:8080")?.run().await
    // println!("Started http server: 127.0.0.1:8080");
}


