#![allow(proc_macro_derive_resolution_fallback)]

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
