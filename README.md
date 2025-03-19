## The Rust Sql Toolkit
[SQLx](https://crates.io/crates/sqlx)

[SQLx cli](https://crates.io/crates/sqlx-cli)

## Install SQLx CLI With Rust Tool Chain

```bash
# only for postgres
$ cargo install sqlx-cli --no-default-features --features native-tls,postgres
```
[sqlx cli manual](https://docs.rs/crate/sqlx-cli/0.8.3)

## Conversions between Rust and Postgres types
[map between Rust and Postgres Types](https://docs.rs/sqlx/latest/sqlx/postgres/types/index.html)

## sqlx::query_as vs sqlx::query_as!
sqlx::query_as! is a MICRO and doesn't use FromRow. It require the environment variable `DATABASE_URL` to be set at the build time.
