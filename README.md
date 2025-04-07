# PostgreSQL Demo in Rust
The asynchronous PostgreSQL driver(client) in rust has multiple options:
1. [tokio-postgres](https://docs.rs/tokio-postgres/latest/tokio_postgres/)

    It only works with runtime `tokio`.
1. SQLx

   It is compatible with the async-std, tokio, and actix runtimes and supports postgres, mysql and sqlite, and is more widely recommended.

The ORM(Object-Relational Mapping) and Query builder also has multiple options:
1. [diesel](https://crates.io/crates/diesel)

    It has the most all-time downloads and supports postgres, mysql and sqlite but is not asynchronous.

2. [sea-orm](https://crates.io/crates/sea-orm)

    It is widely recommended and also supports postgres, mysql and sqlite. Plus, it is asynchronous.

So the SQlx and sea-orm are demoed and they are coded in vscode with rust extension.

> After playing the SQLx and Sea-orm as a rust beginner with some experience of writing PLSQL, I think SQLx is good enough to operate the database. It is flexible, convenient  and easy to start with.

> sea-orm is too much: it took me more time to archive the same work done in sqlx_driver_demo.

## SQLx Demo
To build it, the database has to be up first and then apply the database migrations:
```bash
$ sh deploy.sh
$ sh sqlx_driver_demo/db_migrates.sh
```
It demos:
- database migration
- create schema name `kres`
- customized enum mapping between rust and postgres
  ```sql
  CREATE type kres.device_type AS ENUM ('ngc20', 'elexant5010i', 'elexant40x0i');
  ```
  ```rust
    #[derive(Debug,sqlx::Type)]
    #[sqlx(type_name = "kres.device_type", rename_all = "lowercase")]
    pub enum DeviceType {
        NGC20,
        ELEXANT5010I,
        ELEXANT40X0I,
    }
  ```
- customized type mapping between rust and postgres

  Mainly demos the convention between `HundredthTemperatureCelcius` and `bigdecimal`.
  ```sql
    create table kres.Config (
        id serial primary key,
        device_id integer not null,
        control_temperature numeric(5, 2),
        temperature_low_alarm_enable boolean,
        created_at timestamp with time zone not null default current_timestamp,
        foreign key (device_id) references kres.Device(id)
    );
  ```
  ```rust
        pub struct Config {
        pub id: i32,
        pub device_id:i32,
        pub control_temperature: Option<HundredthTemperatureCelcius>,
        pub temperature_low_alarm_enable: Option<bool>,
        pub created_at: OffsetDateTime,
    }
  ```
- query_as and query_as!
    - Implementing the FromRow trait for `Config` which is required by query_as
    - Implementing the Encode and Type trait for `HundredthTemperatureCelcius` which is required by query_as!
### The SQlx Toolkit
[SQLx](https://crates.io/crates/sqlx)

[SQLx cli](https://crates.io/crates/sqlx-cli)

### Install SQLx CLI With Rust Tool Chain

```bash
# only for postgres
$ cargo install sqlx-cli --no-default-features --features native-tls,postgres
```
[sqlx cli manual](https://docs.rs/crate/sqlx-cli/0.8.3)

### Conversions between Rust and Postgres types
[map between Rust and Postgres Types](https://docs.rs/sqlx/latest/sqlx/postgres/types/index.html)

### sqlx::query_as vs sqlx::query_as!
sqlx::query_as! is a MACRO and doesn't use FromRow. It require the environment variable `DATABASE_URL` to be set at the build time.

sqlx::query_as! verifies the sql statement at compile time.

## SeaORM Demo
The database is created and migration is applied in the program, so the `db_migrate.sh` script is not used. It demos:
1. creating database

1. database migration

1. creating schema name `kres`

1. create customized enum and `Device` table  under `kres` in database

1. use the sea-orm-cli-generated entity `Device` to do the insert and select.

### Library and Cli
[sea-orm](https://crates.io/crates/sea-orm)

[sea-orm-cli](https://crates.io/crates/sea-orm-cli)

### Install sea-orm-cli
To install sea-orm-cli, we need to have pkg-config and libssl-dev installed.
```bash
    sudo apt install pkg-config
    sudo apt install libssl-dev
```
> [NOTE]
>
> sea-orm-cli doesn't have a command to create database, so we can either create the database when `docker compose` starts the PostgreSql container by specifying the `POSTGRES_DB` in docker-compose.ymal or create it when the application starts to run.

In this demo, the database is created when the demo application starts.