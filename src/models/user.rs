use chrono::NaiveDateTime;
use diesel::{Queryable, Insertable};
#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    #[serde(skip)]
    pub id: i32,
    pub email: String,
    pub company: String,
    #[serde(skip)]
    pub password: String,
    pub created_at: NaiveDateTime,

}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub company: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

use bcrypt::{hash, DEFAULT_COST, verify};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use chrono::Local;
use diesel::associations::HasTable;
use crate::db_connection::PgPooledConnection;
use crate::errors::MyStoreError;
use crate::schema::users;

impl User {
    pub fn create(register_user: RegisterUser, conn: &mut PgConnection) -> Result<User, MyStoreError> {
        Ok(diesel::insert_into(users::table)
            .values(NewUser {
                email: register_user.email,
                company: register_user.company,
                password: Self::hash_password(register_user.password)?,
                created_at: Local::now().naive_local(),
            }).get_result(conn)?
        )
    }
    pub fn hash_password(plain: String) -> Result<String, MyStoreError> {
        Ok(hash(plain, DEFAULT_COST)?)
    }
}

#[derive(Deserialize)]
pub struct RegisterUser {
    pub email: String,
    pub company: String,
    pub password: String,
    pub password_confirmation: String,
}

impl RegisterUser {
    pub fn validate(self) -> Result<RegisterUser, MyStoreError> {
        if self.password == self.password_confirmation {
            Ok(self)
        } else {
            Err(MyStoreError::PasswordNotMatch("password and password confirmation does not match".parse().unwrap()))
        }
    }
}

#[derive(Deserialize)]
pub struct AuthUser {
    pub email: String,
    pub password: String,
}

impl AuthUser {
    pub fn login(&self, conn: &mut PgConnection) -> Result<User, MyStoreError> {
        use crate::schema::users::dsl::email;
        let mut records = (users::table).filter(email.eq(&self.email))
            .load::<User>(conn)?;
        let user = records.pop().ok_or(MyStoreError::DBError(diesel::result::Error::NotFound))?;
        let verify_password = verify(&self.password, &user.password).map_err(|_err| MyStoreError::WrongPassword("Wrong password. please check again".to_string()))?;
        if verify_password {
            Ok(user)
        } else {
            Err(MyStoreError::WrongPassword("Wrong password. please check again".to_string()))
        }
    }
}