
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbBackend, DbErr, Statement, Set, entity::*};
//use sea_orm_migration::prelude::*;
use migration::{Migrator, MigratorTrait};

const DBSERVER_URL: &str = "postgresql://postgres:123456@localhost:5432";
const DB_NAME: &str = "seaorm_demo";

mod entities;
use entities::{device, sea_orm_active_enums::DeviceType};


#[tokio::main]
async fn main() {

    let db = create_database_if_not_exist(DBSERVER_URL, DB_NAME).await;
    let db = match db {
        Ok(db_conn) => {
            println!("Database connected: {:?}", db_conn);
            db_conn
        },
        Err(e) => {
            println!("Failed to create database: {:?}", e);
            return;
        }
    };
    db.ping().await.expect("Failed to ping database");
    Migrator::up(&db, None).await.expect("Failed to run migrations");

    // Insert a new device
    let dev = device::ActiveModel {
        device_type: Set(DeviceType::Elexant40x0i.to_owned()),
        tag: Set("NGC20".to_owned()),
        vendor: Set(Some("Chemelex".to_owned())),
        ..Default::default()
    };
    let res = device::Entity::insert(dev).exec(&db).await;
    match res {
        Ok(_) => println!("Device inserted successfully"),
        Err(e) => println!("Failed to insert device: {:?}", e),
    }

    // select all devices
    let devices = device::Entity::find().all(&db).await;
    match devices {
        Ok(devices) => {
            println!("Devices: {:?}", devices);
        },
        Err(e) => {
            println!("Failed to select devices: {:?}", e);
        }
    }

    //println!("Database connected: {:?}", db);
    db.close().await.expect("Failed to close database");
}

async fn create_database_if_not_exist(dberver_url:&str, db_name:&str) -> Result<DatabaseConnection, DbErr> {
    let db_url = &format!("{}/{}", dberver_url, db_name);
    let db = Database::connect(db_url).await;
    if let Ok(conn) = db {
        return Ok(conn);
    }

    // here we have error, check if it is a connection error
    if let Err(err) = db {
       match err {
           DbErr::Conn(e) => {
            println!("database '{}' not exist: {:?}", db_name, e);
           },
           _ => return Err(err),
       }
    }

    // create database
    let db = Database::connect(dberver_url).await;
    if let Err(e) = db {
        return Err(e);
    }

    let db = db.unwrap();
    let db_backend = db.get_database_backend();

    if db_backend != DbBackend::Postgres {
        return Err(DbErr::Custom("Unsupported database backend".to_string()));
    }

    db.execute(Statement::from_string(db.get_database_backend(), format!("CREATE DATABASE {};", db_name))).await?;
    Database::connect(db_url).await
}