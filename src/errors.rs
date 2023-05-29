use std::fmt;
use actix_web::{HttpResponse, ResponseError};
use actix_web::body::BoxBody;
use bcrypt::BcryptError;
use csrf::CsrfError;
use diesel::result;

#[derive(Debug)]
pub enum MyStoreError{
    HashError(BcryptError),
    DBError(result::Error),
    PasswordNotMatch(String),
    WrongPassword(String),
    CsrfError(CsrfError),
}
impl From<BcryptError> for MyStoreError{
    fn from(e: BcryptError) -> Self {
        MyStoreError::HashError(e)
    }
}
impl From<result::Error> for MyStoreError {
    fn from(error: result::Error) -> Self {
        MyStoreError::DBError(error)
    }
}
impl From<CsrfError> for MyStoreError{
    fn from(error:CsrfError)->Self{MyStoreError::CsrfError(error)}
}
impl ResponseError for MyStoreError{
    fn error_response(&self) -> HttpResponse<BoxBody> {
        use ServiceError::*;
        match self {
            MyStoreError::DBError(_) | MyStoreError::HashError(_) => HttpResponse::InternalServerError().json(res),
            MyStoreError::CsrfError(_) | MyStoreError::WrongPassword(_) | MyStoreError::PasswordNotMatch(_) => HttpResponse::BadRequest().json(res),
        }
    }
}
impl fmt::Display for MyStoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyStoreError::HashError(error) => write!(f, "{}", error),
            MyStoreError::DBError(error) => write!(f, "{}", error),
            MyStoreError::PasswordNotMatch(error) => write!(f, "{}", error),
            MyStoreError::WrongPassword(error) => write!(f, "{}", error)
            MyStoreError::CsrfError(error) => write!(f, "{}", error)
        }
    }
}
