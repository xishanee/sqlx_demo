pub mod device;
pub mod config;
pub mod kres_types;
pub mod config_ex;

use device::{Device, DeviceType, EntityId};
use config::Config;
use config_ex::ConfigEx;
use kres_types::{HundredthTemperatureCelcius, Optional};
use sqlx::postgres;
use fake::{faker::{name::en::FirstName, company::en::CompanyName}, Fake};
use std::str::FromStr;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("DATABASE_URL: {db_url}");

    let pool = postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await;

    let pool = match pool {
        Ok(pool) => {
            println!("Connected to database");
            pool
        }
        Err(e) => {
            eprintln!("Error connecting to database: {e}");
            return;
        }
    };

    //FIXME the query_as doesn't work
    // let result = sqlx::query_as::<_, Device>(
    //     r#"select id, device_type AS "device_type!: model::DeviceType", tag, vendor, created_at from kres.device"#)
    //     .fetch_all(&pool)
    //     .await;

    // Note: query_as!(micro) doesn't not use sqlx::FromRow
    let result = sqlx::query_as!(Device,
        r#"select id, device_type AS "device_type!: device::DeviceType", tag, vendor, created_at from kres.device"#)
        .fetch_all(&pool)
        .await;

    let devices = match result {
        Ok(devs) => devs,
        Err(e) => {
            eprintln!("Error fetching data: {e}");
            return
        }
    };

    println!("{} devices fetched.", devices.len());
    for device in devices {
        println!("device: {:?}", device)
    }

    // It works
    // let insert_result = sqlx::query_as::<_, EntityId>(
    //     "insert into kres.device (device_type, tag, vendor) values($1, $2, $3) returning id")
    //     .bind(DeviceType::NGC20)
    //     .bind("ngc20-dev-2".to_string())
    //     .bind("Chemelex".to_string())
    //     .fetch_one(&pool)
    //     .await;

    let insert_result = sqlx::query_as!(
        EntityId,
        r#"insert into kres.device (device_type, tag, vendor) values($1, $2, $3) returning id"#,
        DeviceType::NGC20 as DeviceType,
        format!("ngc20-{}",FirstName().fake::<String>()),
        CompanyName().fake::<String>())
        .fetch_one(&pool)
        .await;

    let result = match insert_result {
        Ok(id) => id,
        Err(e) => {
            eprintln!("Error inserting data: {e}");
            return;
        }
    };
    println!("id:{:?}", result);

    let configs = sqlx::query_as::<_,Config>("select * from kres.config")
        .fetch_all(&pool)
        .await;

    let configs = match configs {
        Ok(cfgs) => cfgs,
        Err(e) => {
            eprintln!("Error fetching data from kes.config: {e}");
            return;
        }
    };

    for config in configs {
        println!("{:?}", config);
    }

    let temp = HundredthTemperatureCelcius::from_str("-125.00°C").unwrap();
    let post_config = Config {
        id: 1,
        device_id: 1,
        control_temperature: Some(temp),
        temperature_low_alarm_enable: Some(true),
        created_at: time::OffsetDateTime::now_utc(),
    };

    let insert_result: Result<Config, sqlx::Error> = sqlx::query_as(
        r#"insert into kres.config (device_id, control_temperature, temperature_low_alarm_enable) values($1, $2, $3)
        returning id, device_id, control_temperature, temperature_low_alarm_enable, created_at"#,
        )
        .bind(post_config.device_id)
        //.bind(post_config.control_temperature.map(|t| t.value))
        .bind(post_config.control_temperature) // use this line instead of the above line requires the implementation of Trait (Type and Encode) for the HundredthTemperatureCelcius
        .bind(post_config.temperature_low_alarm_enable)
        .fetch_one(&pool)
        .await;

    let added_config = match insert_result {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Error inserting data: {e}");
            return;
        }
    };

    println!("Added config: {:?}", added_config);

    let configs = sqlx::query_as!(ConfigEx, "select device_id, control_temperature, temperature_low_alarm_enable from kres.config")
        .fetch_all(&pool)
        .await;

    if let Ok(cfgs) = configs {
        for config in cfgs {
            println!("{:?}", config);
        }
    } else {
        eprintln!("Error fetching data from kes.config");
    }

    let temp = HundredthTemperatureCelcius::from_str("-35.00°C").unwrap();
    let post_config = ConfigEx {
        device_id: 1,
        //control_temperature: Optional(None),
        control_temperature: Optional(Some(temp)),
        temperature_low_alarm_enable: Some(true),
    };

    let insert_result = sqlx::query_as!(
        EntityId,
        r#"insert into kres.config (device_id, control_temperature, temperature_low_alarm_enable) values($1, $2, $3) returning id"#,
        post_config.device_id,
        post_config.control_temperature.0.map(|t| t.value),
        post_config.temperature_low_alarm_enable)
        .fetch_one(&pool)
        .await;
    let result = match insert_result {
        Ok(id) => id,
        Err(e) => {
            eprintln!("Error inserting data: {e}");
            return;
        }
    };
    println!("id:{:?}", result);
    pool.close().await;
}
