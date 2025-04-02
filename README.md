# SQLx Demo
Source code is in sqlx_driver_demo.
## The Rust Sql Toolkit
[SQLx](https://crates.io/crates/sqlx)

[SQLx cli](https://crates.io/crates/sqlx-cli)

SQLx is compatible with the async-std, tokio, and actix runtimes.

## Install SQLx CLI With Rust Tool Chain

```bash
# only for postgres
$ cargo install sqlx-cli --no-default-features --features native-tls,postgres
```
[sqlx cli manual](https://docs.rs/crate/sqlx-cli/0.8.3)

## Conversions between Rust and Postgres types
[map between Rust and Postgres Types](https://docs.rs/sqlx/latest/sqlx/postgres/types/index.html)

## sqlx::query_as vs sqlx::query_as!
sqlx::query_as! is a MACRO and doesn't use FromRow. It require the environment variable `DATABASE_URL` to be set at the build time.

sqlx::query_as! verifies the sql statement at compile time.

# SeaORM Demo
The source code is in seaorm_demo.

## Library and Cli
[sea-orm](https://crates.io/crates/sea-orm)

[sea-orm-cli](https://crates.io/crates/sea-orm-cli)

## Install sea-orm-cli
To install sea-orm-cli, we need to have pkg-config and libssl-dev installed.
```bash
    sudo apt install pkg-config
    sudo apt install libssl-dev
```
> [!NOTE]
>
> sea-orm-cli doesn't have a command to create database, so we can either create the database when `docker compose` starts the PostgreSql container by specifying the `POSTGRES_DB` in docker-compose.ymal or create it when the application starts to run.

In this demo, the database is created when the demo application starts.