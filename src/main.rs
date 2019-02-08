#[macro_use]
extern crate diesel;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::RunQueryDsl;
use uuid::Uuid;

mod schema;

use crate::schema::*;

//#[derive(Debug, Queryable, Insertable)]
//#[table_name = "test_table"]
//pub struct TestTable {
//    pub id: Uuid,
//    pub i2: i16,
//    pub i4: i32,
//    pub i8: i64,
//    pub nullable_i2: Option<i16>,
//    pub nullable_i4: Option<i32>,
//    pub nullable_i8: Option<i64>,
//}

#[derive(Debug, Queryable, Insertable)]
#[table_name = "test_table"]
pub struct TestTable {
    pub id: Uuid,
    pub i2: i64,
    pub i4: i32,
    pub i8: i64,
    pub nullable_i2: Option<i64>,
    pub nullable_i4: Option<i32>,
    pub nullable_i8: Option<i64>,
}

fn main() {
    let manager = ConnectionManager::<PgConnection>::new(
        "postgres://root:@localhost:26257/test_cockroach_diesel",
    );
    let pool = Pool::new(manager).unwrap();

    let conn = pool.get().unwrap();

    let results = test_table::table.load::<TestTable>(&conn).unwrap();

    println!("RESULTS: {:?}", &results);
}
