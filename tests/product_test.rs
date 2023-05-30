#[macro_use]
extern crate dotenv_codegen;
mod common;
mod test{
    use std::io::Read;
    use actix_http::{HttpService, Request};

    use actix_web::{http,App,web,test};
    use std::{env, str};
    use std::cell::RefMut;
    use std::sync::Arc;
    use actix_http::body::BoxBody;
    use actix_http::header::CONTENT_TYPE;
    use actix_web::dev::{Service, ServiceResponse};
    use actix_web::http::header::ContentType;
    use csrf::AesGcmCsrfProtection;
    use diesel::{CombineDsl, RunQueryDsl};
    use futures::TryFutureExt;
    use hex::decode;
    use mystore_lib::schema::users::dsl::users;
    use crate::common::db_connection::establish_connection;
    async fn get_app() -> impl Service<Request, Error = actix_web::Error> {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(establish_connection()))
                .route("/products", web::get().to(::mystore_lib::handlers::products::index))
        ).await;
        app
    }
    #[actix_web::test]
    async fn test(){
        env::set_var("RUST_BACKTRACE", "full");
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(establish_connection()))
                .route("/products", web::get().to(::mystore_lib::handlers::products::index)
                )
        ).await;


        let req = test::TestRequest::get().uri("/products").insert_header(ContentType::json())
            .to_request();
        let resp = test::call_service(&app, req).await;
        println!("{}",resp.status());
        assert!(resp.status().is_success());
        assert_eq!(
            resp.headers().get(http::header::CONTENT_TYPE).unwrap(),
            "application/json"
        );
        let bytes = test::read_body(resp).await;
        let body = str::from_utf8(&bytes).unwrap();
        assert_eq!(body, "[]");

    }
    fn create_user(){
        use diesel::RunQueryDsl;
        use ::mystore_lib::schema::users;
        use ::mystore_lib::models::user::{ NewUser, User };
        use chrono::Local;
        let conn = establish_connection();
        let mut pg_pool = conn.get().unwrap();
        diesel::delete(users::table).execute(&mut pg_pool).unwrap();
        diesel::insert_into(users::table)
            .values(NewUser{
                email:"john@doe.com".to_string(),
                company:"johndoentertainment".to_string(),
                password: User::hash_password("1234568".to_string()).unwrap(),
                created_at: Local::now().naive_local()
            })
            .get_result::<User>(&mut pg_pool).unwrap();
    }
    async fn login(){
        let request =  test::TestRequest::post().uri("/auth").insert_header(ContentType::json()).to_request();
        let app = get_app().await;

    }
}