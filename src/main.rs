#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate diesel;

use diesel::pg::PgConnection;
use diesel::{Connection, RunQueryDsl};
use uuid::Uuid;

const DB_URL: &str = "postgres://root:@localhost:26257/test_cockroach_diesel";

table! {
    test_table (id) {
        id -> Uuid,
        i2 -> Int2,
        i4 -> Int4,
        i8 -> Int8,
        nullable_i2 -> Nullable<Int2>,
        nullable_i4 -> Nullable<Int4>,
        nullable_i8 -> Nullable<Int8>,
    }
}

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

// Issue can be fixed by switching to this instead:

//table! {
//    test_table (id) {
//        id -> Uuid,
//        i2 -> Int8,
//        i4 -> Int8,
//        i8 -> Int8,
//        nullable_i2 -> Nullable<Int8>,
//        nullable_i4 -> Nullable<Int8>,
//        nullable_i8 -> Nullable<Int8>,
//    }
//}
//
//#[derive(Debug, Queryable, Insertable)]
//#[table_name = "test_table"]
//pub struct TestTable {
//    pub id: Uuid,
//    pub i2: i64,
//    pub i4: i64,
//    pub i8: i64,
//    pub nullable_i2: Option<i64>,
//    pub nullable_i4: Option<i64>,
//    pub nullable_i8: Option<i64>,
//}

fn main() {
    let conn = PgConnection::establish(DB_URL).unwrap();

    let results = test_table::table.load::<TestTable>(&conn).unwrap();

    println!("RESULTS: {:?}", &results);
}
