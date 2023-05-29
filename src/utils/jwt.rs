use actix_web::error::ErrorInternalServerError;
use actix_web::HttpResponse;
use bcrypt::BcryptError;
use chrono::{Duration, Local};
use data_encoding::DecodeError;

use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
use crate::errors::MyStoreError;


#[derive(Debug, Serialize, Deserialize)]
struct Claims{
    sub:String,
    company:String,
    exp:usize,
}
pub struct SlimUser{
    pub email:String,
    pub company:String,
}
impl From<Claims> for SlimUser{
    fn from(claim: Claims) -> Self {
        Self{
            email:claim.sub,
            company:claim.company,
        }
    }
}
impl Claims{
    fn with_email(email:&str,company:&str)->Self{
        Self{
            sub:email.into(),
            company:company.into(),
            exp:(Local::now()+Duration::hours(24)).timestamp() as usize
        }
    }
}
pub fn create_token(email:&str, company:&str)->Result<String,MyStoreError>{
    let claims = Claims::with_email(email,company);
    encode(&Header::default(), &claims, &EncodingKey::from_secret(get_secret()))
        .map_err(|e|MyStoreError::HashError(BcryptError::InvalidHash(e.to_string())))
}
pub fn decode_token(token: &str) -> Result<SlimUser, MyStoreError> {
    decode::<Claims>(token, &DecodingKey::from_secret(get_secret()), &Validation::default())
        .map(|data| data.claims.into())
        .map_err(|e| MyStoreError::HashError(BcryptError::InvalidHash("".to_string())))
}
fn get_secret<'a>()->&'a[u8]{
    dotenv!("JWT_SECRET").as_bytes()
}