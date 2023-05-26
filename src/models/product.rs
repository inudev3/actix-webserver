use diesel::{AsChangeset, PgConnection, QueryDsl, RunQueryDsl};
use crate::db_connection::{establish_connection, PgPooledConnection};
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

    pub fn find(id:&i32, conn:&mut PgPooledConnection)->Result<Product, diesel::result::Error>{
        products::table.find(id).first(conn)
    }
    pub fn destroy(id:&i32, conn:&mut PgPooledConnection)->Result<(), diesel::result::Error>{
        diesel::delete(dsl::products.find(id)).execute(conn)?;
        Ok(())
    }
    pub fn update(id:&i32, new_product:&NewProduct, conn:&mut PgPooledConnection)->Result<(),diesel::result::Error>{


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
    pub fn create(&self, conn:&mut PgPooledConnection)->Result<Product, diesel::result::Error>{
        use diesel::RunQueryDsl;


        diesel::insert_into(products::table).values(self).get_result(conn)
    }
}

#[derive(Serialize,Deserialize)]
pub struct ProductList(Vec<Product>); //newtype-pattern, we can add any trait.
impl ProductList{
    pub fn list(conn:&mut PgPooledConnection)->Self{
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;
        use crate::schema::products::dsl::*;


        let res = products.limit(10)
            .load::<Product>(conn)
            .expect("Error loading products");
        Self(res)
    }
}