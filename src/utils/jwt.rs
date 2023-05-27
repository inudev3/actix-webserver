use actix_web::error::ErrorInternalServerError;
use chrono::{Duration, Local};
#[macro_use]
extern crate dotenv_codegen;
use jsonwebtoken::{encode, EncodingKey, Header};


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
impl From<Claim> for SlimUser{
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
fn get_secret<'a>()->&'a[u8]{
    dotenv!("JWT_SECRET").as_bytes()
}