use std::future::{Future};
use futures::future::{Ready,ok,err};
use actix_identity::Identity;
use actix_web::{FromRequest, HttpRequest, HttpResponse, web};
use actix_web::dev::Payload;
use actix_web::error::ErrorInternalServerError;
use csrf::{AesGcmCsrfProtection, CsrfProtection};
use crate::db_connection::{PgPool, PgPooledConnection};
use crate::utils::jwt::{decode_token, SlimUser};

use data_encoding::BASE64;
pub mod products;
pub mod register;
pub mod authentication;
pub type LoggedUser = SlimUser;
impl FromRequest for LoggedUser{
    type Error = actix_web::Error;
    type Future = Ready<Result<Self,actix_web::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let protect = req.app_data::<AesGcmCsrfProtection>()
            .ok_or_else(|e|ErrorInternalServerError(e))?;
        let csrf_token = req.headers().get("X-CSRF-TOKEN").ok_or(HttpResponse::Unauthorized())?;
        let csrf_cookie = req.cookie("csrf_cookie").ok_or(HttpResponse::Unauthorized())?.value();


        let token_bytes = BASE64.decode(csrf_token.as_bytes()).map_err(|e|HttpResponse::InternalServerError().json(e.to_string()))?;
        let cookie_bytes = BASE64.decode(csrf_cookie.as_bytes()).map_err(|e|HttpResponse::InternalServerError().json(e.to_string()))?;
        let parsed_token = protect.parse_token(&token_bytes).map_err(|e|HttpResponse::InternalServerError().json(e.to_string()))?;
        let parsed_cookie = protect.parse_cookie(&cookie_bytes).map_err(|e|HttpResponse::InternalServerError().json(e.to_string()))?;
        if !protect.verify_token_pair(&parsed_token, &parsed_cookie){
            return Err(HttpResponse::Unauthorized().finish().into())
        }
        if let Some(identity) = Identity::from_request(req, payload).into_inner()?.identity() {
            let user: SlimUser = decode_token(&identity)?;
            return Ok(user as LoggedUser);
        }
        Err(HttpResponse::Unauthorized().finish().into())
    }
}

pub fn pg_pool_handler(pool: web::Data<PgPool>) -> actix_web::Result<PgPooledConnection> {

    pool.get()
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(e.to_string())
        })
}

