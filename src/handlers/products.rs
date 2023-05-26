use std::fmt::{Display, Formatter};
use std::io::Stderr;
use std::ops::{Deref, DerefMut};
use actix_web;
use actix_web::{Error, error, HttpRequest, HttpResponse, web};
use actix_web::error::{HttpError, PayloadError};
use actix_web::web::Json;
use diesel::Insertable;
use diesel::pg::Pg;
use crate::db_connection::{PgPool, PgPooledConnection};
use crate::models::product::{NewProduct, Product, ProductList};

fn pg_pool_handler(pool: web::Data<PgPool>) -> Result<PgPooledConnection, HttpResponse> {

    pool.get()
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

pub async fn index(_req: HttpRequest, pool:web::Data<PgPool>) -> HttpResponse {
    let mut pg_pool = pg_pool_handler(pool).unwrap();
    HttpResponse::Ok().json(ProductList::list( &mut pg_pool))
}


pub async fn create(new_product: Json<NewProduct>,pool:web::Data<PgPool>) -> HttpResponse {
    let mut pg_pool = pg_pool_handler(pool).unwrap();
    new_product.create(&mut pg_pool)
        .map(|product| HttpResponse::Ok().json(product))

        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string())).unwrap()
}

pub async fn show(id: web::Path<i32>,pool:web::Data<PgPool>) -> HttpResponse {
    let mut pg_pool = pg_pool_handler(pool).unwrap();
    Product::find(&id, &mut pg_pool).map(|product| HttpResponse::Ok().json(product))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        }).unwrap()
}

pub async fn destroy(id: web::Path<i32>,pool:web::Data<PgPool>) -> HttpResponse {
    let mut pg_pool = pg_pool_handler(pool).unwrap();
    Product::destroy(&id, &mut pg_pool).map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string())).unwrap()
}

pub async fn update(id: web::Path<i32>, new_product: Json<NewProduct>,pool:web::Data<PgPool>) -> HttpResponse {
    let mut pg_pool = pg_pool_handler(pool).unwrap();
    Product::update(&id, &new_product, &mut pg_pool).map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string())).unwrap()
}