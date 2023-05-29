use actix_web::{HttpResponse, web};
use actix_web::error::ErrorInternalServerError;
use crate::db_connection::PgPool;
use crate::handlers::pg_pool_handler;
use crate::models::user::{RegisterUser, User};


pub async fn register(new_user: web::Json<RegisterUser>, pool: web::Data<PgPool>)->actix_web::Result<HttpResponse>{
    let mut pg_pool = pg_pool_handler(pool)?;
    let register_user = new_user
        .into_inner()
        .validate()
        .map_err(|e|{
            ErrorInternalServerError(e)
        })?;
    User::create(register_user, &mut pg_pool).map(|user|HttpResponse::Ok().json(user))
        .map_err(|e|ErrorInternalServerError(e))
}