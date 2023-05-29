use std::fmt::{Display, Formatter};
use std::io::Stderr;
use std::ops::{Deref, DerefMut};
use actix_web;
use actix_web::{Error, error, HttpRequest, HttpResponse, web};
use actix_web::error::{ErrorInternalServerError, HttpError, PayloadError};
use actix_web::web::Json;
use diesel::Insertable;
use diesel::pg::Pg;
use crate::db_connection::{PgPool, PgPooledConnection};
use crate::handlers::pg_pool_handler;
use crate::models::product::{NewProduct, Product, ProductList};

pub async fn index(_req: HttpRequest, pool:web::Data<PgPool>) ->actix_web::Result<HttpResponse> {
    let mut pg_pool = pg_pool_handler(pool)?;
    Ok(HttpResponse::Ok().json(ProductList::list( &mut pg_pool)))
}


pub async fn create(new_product: Json<NewProduct>,pool:web::Data<PgPool>) -> actix_web::Result<HttpResponse> {
    let mut pg_pool = pg_pool_handler(pool)?;
    new_product.create(&mut pg_pool)
        .map(|product| HttpResponse::Ok().json(product))

        .map_err(|e| ErrorInternalServerError(e))
}

pub async fn show(id: web::Path<i32>,pool:web::Data<PgPool>) -> actix_web::Result<HttpResponse> {
    let mut pg_pool = pg_pool_handler(pool)?;
    Product::find(&id, &mut pg_pool).map(|product| HttpResponse::Ok().json(product))
        .map_err(|e| {
            ErrorInternalServerError(e)
        })
}

pub async fn destroy(id: web::Path<i32>,pool:web::Data<PgPool>) -> actix_web::Result<HttpResponse> {
    let mut pg_pool = pg_pool_handler(pool)?;
    Product::destroy(&id, &mut pg_pool).map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| ErrorInternalServerError(e))
}

pub async fn update(id: web::Path<i32>, new_product: Json<NewProduct>,pool:web::Data<PgPool>) -> actix_web::Result<HttpResponse> {
    let mut pg_pool = pg_pool_handler(pool)?;
    Product::update(&id, &new_product, &mut pg_pool).map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| ErrorInternalServerError(e))
}