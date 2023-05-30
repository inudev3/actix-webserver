use diesel::{AsChangeset, Queryable, Insertable,PgConnection, QueryDsl, RunQueryDsl};
use diesel::associations::HasTable;
use crate::db_connection::{establish_connection, PgPooledConnection};
use crate::errors::MyStoreError;
use crate::schema::products;
use crate::schema::products::dsl;
use serde::{Serialize,Deserialize};

#[derive(Queryable,Serialize,Deserialize)]
pub struct Product{
    pub id:i32,
    pub name:String,
    pub stock:f64,
    pub price:Option<i32>,
}
impl Product{

    pub fn find(id:&i32, conn:&mut PgConnection)->Result<Product, MyStoreError>{

        Ok(products::table.find(id).first(conn)?)
    }
    pub fn destroy(id:&i32, conn:&mut PgConnection)->Result<(), MyStoreError>{
        diesel::delete(dsl::products.find(id)).execute(conn)?;
        Ok(())
    }
    pub fn update(id:&i32, new_product:&NewProduct, conn:&mut PgConnection)->Result<(),MyStoreError>{


        diesel::update(dsl::products.find(id))
            .set(new_product)
            .execute(conn)?;

        Ok(())
    }
}
#[derive(Insertable,Deserialize,AsChangeset)]
#[table_name="products"]
pub struct NewProduct{
    pub name:Option<String>,
    pub stock:Option<f64>,
    pub price:Option<i32>,
}

impl NewProduct{
    pub fn create(&self, conn:&mut PgConnection)->Result<Product, MyStoreError>{
        use diesel::RunQueryDsl;


        Ok(diesel::insert_into(products::table).values(self).get_result(conn)?)
    }
}

#[derive(Serialize,Deserialize)]
pub struct ProductList(Vec<Product>); //newtype-pattern, we can add any trait.
impl ProductList{
    pub fn list(conn:&mut PgConnection)->Self{
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;


        let res = dsl::products.limit(10)
            .load::<Product>(conn)
            .expect("Error loading products");
        Self(res)
    }
}