# Steps to run

While having a Cockroach instance running locally, create the DB & table & import the test data:

`cockroach sql --insecure < schema.sql`

With a decently-recent versin of Rust installed, run the code:

`cargo run`