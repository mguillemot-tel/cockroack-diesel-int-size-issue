#[macro_use]
extern crate diesel;

use diesel::pg::PgConnection;
use diesel::{Connection, RunQueryDsl};
use uuid::Uuid;

mod schema;

use crate::schema::*;

const DB_URL: &str = "postgres://root:@localhost:26257/test_cockroach_diesel";

#[derive(Debug, Queryable, Insertable)]
#[table_name = "test_table"]
pub struct TestTable {
    pub id: Uuid,
    pub i2: i16,
    pub i4: i32,
    pub i8: i64,
    pub nullable_i2: Option<i16>,
    pub nullable_i4: Option<i32>,
    pub nullable_i8: Option<i64>,
}

fn main() {
    let conn = PgConnection::establish(DB_URL).unwrap();

    let results = test_table::table.load::<TestTable>(&conn).unwrap();

    println!("RESULTS: {:?}", &results);
}
