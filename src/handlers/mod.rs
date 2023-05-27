use actix_web::web;
use crate::db_connection::{PgPool, PgPooledConnection};

pub mod products;
pub mod register;
pub mod authentication;


pub fn pg_pool_handler(pool: web::Data<PgPool>) -> actix_web::Result<PgPooledConnection> {

    pool.get()
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(e.to_string())
        })
}

