use actix_identity::Identity;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, web};
use actix_web::cookie::Cookie;
use actix_web::error::{ErrorInternalServerError, ErrorNotFound};
use actix_web::http::header;
use csrf::{AesGcmCsrfProtection, CsrfProtection};
use crate::models::user::AuthUser;
use hex;

use crate::db_connection::PgPool;
use crate::errors::MyStoreError;
use crate::handlers::pg_pool_handler;
use crate::utils::jwt::create_token;


pub async fn login(auth_user: web::Json<AuthUser>, id:Identity,request:HttpRequest, pool:web::Data<PgPool>,protect:web::Data<AesGcmCsrfProtection>) ->Result<HttpResponse,actix_web::Error>{
    let mut pg_pool = pg_pool_handler(pool)?;
    let user = auth_user
        .login(&mut pg_pool)
        .map_err(|e|{
            match e {
                MyStoreError::DBError(diesel::result::Error::NotFound) => ErrorNotFound(e),
                _ => ErrorInternalServerError(e)
            }})?;

    let token:String = create_token(&user.email, &user.company)?;
    id.remember(token);
    let (csrftoken, csrfcookie) = protect.generate_token_pair(None, 300).map_err(|e|MyStoreError::CsrfError(e))?;

    let cookie = Cookie::build("csrf-cookie", csrfcookie.b64_string())
        .http_only(true)
        .finish();
    let response = HttpResponse::Ok()
        .insert_header(("X-CSRF-TOKEN",csrftoken.b64_string()))
        .cookie(cookie)
        .json(user);
    Ok(response)
}
pub async fn logout(id:Identity)->actix_web::Result<HttpResponse>{
    id.forget();
    Ok(HttpResponse::Ok().finish())
}