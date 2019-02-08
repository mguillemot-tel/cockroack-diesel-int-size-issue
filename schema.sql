DROP DATABASE IF EXISTS test_cockroach_diesel;
CREATE DATABASE test_cockroach_diesel;

USE test_cockroach_diesel;

CREATE TABLE IF NOT EXISTS test_table (
    id UUID PRIMARY KEY,
    i2 INT2 NOT NULL,
    i4 INT4 NOT NULL,
    i8 INT8 NOT NULL,
    nullable_i2 INT2,
    nullable_i4 INT4,
    nullable_i8 INT8
);

INSERT INTO test_table VALUES ('3d917be9-2315-454b-9f05-b4e46e1c84ae', 1, 2, 3, 4, 5, 6);
INSERT INTO test_table VALUES ('fc5c977d-d826-450b-9f2b-edb7680f9d14', 10, 20, 30, NULL, NULL, NULL);
