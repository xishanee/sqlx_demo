
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbBackend, DbErr, Statement};

const DBSERVER_URL: &str = "postgresql://postgres:123456@localhost:5432";
const DB_NAME: &str = "seaorm_demo";

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