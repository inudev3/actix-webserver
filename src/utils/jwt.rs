use actix_web::error::ErrorInternalServerError;
use actix_web::HttpResponse;
use chrono::{Duration, Local};

use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};


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
pub fn create_token(email:&str, company:&str)->Result<String,actix_web::Error>{
    let claims = Claims::with_email(email,company);
    encode(&Header::default(), &claims, &EncodingKey::from_secret(get_secret()))
        .map_err(|e|ErrorInternalServerError(e))
}
pub fn decode_token(token: &str) -> Result<SlimUser, HttpResponse> {
    decode::<Claims>(token, &DecodingKey::from_secret(get_secret()), &Validation::default())
        .map(|data| data.claims.into())
        .map_err(|e| HttpResponse::Unauthorized().json(e.to_string()))
}
fn get_secret<'a>()->&'a[u8]{
    dotenv!("JWT_SECRET").as_bytes()
}