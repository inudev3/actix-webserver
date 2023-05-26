use chrono::NaiveDateTime;
#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User{
    #[serde(skip)]
    pub id:i32,
    pub email:String,
    pub company:String,
    #[serde(skip)]
    pub password:String,
    pub created_at:NaiveDateTime,

}
#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser{
    pub email: String,
    pub company: String,
    pub password: String,
    pub created_at: NaiveDateTime
}
use bcrypt::{hash, DEFAULT_COST};
use diesel::PgConnection;
use chrono::Local;
use crate::errors::MyStoreError;