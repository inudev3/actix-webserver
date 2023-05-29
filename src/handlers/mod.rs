use std::future::{Future};
use futures::future::{Ready,ok,err};
use actix_identity::Identity;
use actix_web::{FromRequest, HttpRequest, HttpResponse, web};
use actix_web::dev::Payload;
use actix_web::error::ErrorInternalServerError;
use csrf::{AesGcmCsrfProtection, CsrfError, CsrfProtection};
use crate::db_connection::{PgPool, PgPooledConnection};
use crate::utils::jwt::{decode_token, SlimUser};

use data_encoding::BASE64;
use log::Log;
use crate::errors::MyStoreError;

pub mod products;
pub mod register;
pub mod authentication;
pub type LoggedUser = SlimUser;
impl FromRequest for LoggedUser{
    type Error = actix_web::Error;
    type Future = Ready<Result<Self,actix_web::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let protect = req.app_data::<AesGcmCsrfProtection>()
            .ok_or(ErrorInternalServerError("Protection not found")).expect("");
        let csrf_token = req.headers().get("X-CSRF-TOKEN").ok_or(MyStoreError::CsrfError(CsrfError::ValidationFailure)).expect("");
        let csrf_cookie = req.cookie("csrf_cookie").ok_or_else(||MyStoreError::CsrfError(CsrfError::ValidationFailure)).expect("");


        let token_bytes = BASE64.decode(csrf_token.as_bytes()).map_err(|e| MyStoreError::CsrfError(CsrfError::InternalError)).expect("");
        let cookie_bytes = BASE64.decode(csrf_cookie.value().as_bytes()).map_err(|e| MyStoreError::CsrfError(CsrfError::InternalError)).expect("");
        let parsed_token = protect.parse_token(&token_bytes).map_err(|e| MyStoreError::CsrfError(CsrfError::InternalError)).expect("");
        let parsed_cookie = protect.parse_cookie(&cookie_bytes).map_err(|e| MyStoreError::CsrfError(CsrfError::InternalError)).expect("");
        if !protect.verify_token_pair(&parsed_token, &parsed_cookie){
            return  err(MyStoreError::PasswordNotMatch("Token and Cookie do not match.".into()).into())
        }
        if let Some(identity) = Identity::from_request(req, payload).into_inner().expect("").identity() {
            let user: LoggedUser = decode_token(&identity).expect("");
            return ok(user);
        }
        err(MyStoreError::CsrfError(CsrfError::ValidationFailure).into())
    }
}

pub fn pg_pool_handler(pool: web::Data<PgPool>) -> actix_web::Result<PgPooledConnection> {

    pool.get()
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(e.to_string())
        })
}

