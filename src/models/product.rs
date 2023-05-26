use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use crate::db_connection::establish_connection;
use crate::schema::products;
use crate::schema::products::dsl;


#[derive(Queryable,Serialize,Deserialize)]
pub struct Product{
    pub id:i32,
    pub name:String,
    pub stock:f64,
    pub price:Option<i32>,
}
impl Product{

    pub fn find(id:&i32)->Result<Product, diesel::result::Error>{
        let mut conn = establish_connection();
        products::table.find(id).first(&mut conn)
    }
    pub fn destroy(id:&i32)->Result<(), diesel::result::Error>{
        let mut conn = establish_connection();
        diesel::delete(dsl::products.find(id)).execute(&mut conn)?;
        Ok(())
    }
    pub fn update(id:&i32, new_product:&NewProduct)->Result<(),diesel::result::Error>{
        let mut conn = establish_connection();

        diesel::update(dsl::products.find(id))
            .set(new_product)
            .execute(&mut conn)?;

        Ok(())
    }
}
#[derive(Insertable,Deserialize)]
#[table_name="products"]
pub struct NewProduct{
    pub name:Option<String>,
    pub stock:Option<f64>,
    pub price:Option<i32>,
}
impl NewProduct{
    pub fn create(&self)->Result<Product, diesel::result::Error>{
        use diesel::RunQueryDsl;
        use crate::db_connection::establish_connection;
        let mut conn = establish_connection();
        diesel::insert_into(products::table).values(self).get_result(&mut conn)
    }
}

#[derive(Serialize,Deserialize)]
pub struct ProductList(Vec<Product>); //newtype-pattern, we can add any trait.
impl ProductList{
    pub fn list()->Self{
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;
        use crate::schema::products::dsl::*;
        use crate::db_connection::establish_connection;
        let mut conn = establish_connection();
        let res = products.limit(10)
            .load::<Product>(&mut conn)
            .expect("Error loading products");
        Self(res)
    }
}