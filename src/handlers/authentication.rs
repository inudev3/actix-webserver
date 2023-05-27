use actix_identity::Identity;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, web};
use actix_web::error::ErrorInternalServerError;
use actix_web::http::header;
use crate::models::user::AuthUser;
use hex;
use csrf_token::CsrfTokenGenerator;
use crate::db_connection::PgPool;
use crate::errors::MyStoreError;
use crate::handlers::pg_pool_handler;
use crate::utils::jwt::create_token;


pub async fn login(auth_user: web::Json<AuthUser>, request:HttpRequest, pool:web::Data<PgPool>,generator:web::Data<CsrfTokenGenerator>) ->actix_web::Result<HttpResponse>{
    let mut pg_pool = pg_pool_handler(pool)?;
    let user = auth_user
        .login(&mut pg_pool)
        .map_err(|e|{
            match e {
                MyStoreError::DBError(diesel::result::Error::NotFound) => HttpResponse::NotFound().json(e.to_string())
                _ => ErrorInternalServerError(e)
            }})?;
    let token:String = create_token(&user.email, &user.company)?;
    Identity::login(&request.extensions(), token);
    let response = HttpResponse::Ok()
        .insert_header(("X-CSRF-TOKEN", hex::encode(generator.generate())))
        .json(user);
    Ok(response)
}
pub async fn logout(id:Identity)->actix_web::Result<HttpResponse>{
    id.logout();
    HttpResponse::Ok().into()
}