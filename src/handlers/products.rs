use std::fmt::{Display, Formatter};
use std::io::Stderr;
use std::ops::{Deref, DerefMut};
use actix_web;
use actix_web::{Error, error, HttpRequest, HttpResponse, web};
use actix_web::error::{HttpError, PayloadError};
use actix_web::web::Json;
use diesel::Insertable;
use crate::models::product::{NewProduct, Product, ProductList};

pub async fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json(ProductList::list())
}


pub async fn create(new_product: Json<NewProduct>) -> HttpResponse {
    new_product.create()
        .map(|product| HttpResponse::Ok().json(product))

        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string())).unwrap()
}

pub async fn show(id: web::Path<i32>) -> HttpResponse {
    Product::find(&id).map(|product| HttpResponse::Ok().json(product))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        }).unwrap()
}

pub async fn destroy(id: web::Path<i32>) -> HttpResponse {
    Product::destroy(&id).map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string())).unwrap()
}

pub async fn update(id: web::Path<i32>, new_product: Json<NewProduct>) -> HttpResponse {
    Product::update(&id, &new_product).map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string())).unwrap()
}