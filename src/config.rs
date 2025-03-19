use sqlx::{FromRow,Row};
use sqlx::postgres::PgRow;
use time::OffsetDateTime;
use sqlx::types::BigDecimal;

use crate::kres_types::HundredthTemperatureCelcius;


#[derive(Debug)]
pub struct Config {
    pub id: i32,
    pub device_id:i32,
    pub control_temperature: Option<HundredthTemperatureCelcius>,
    pub temperature_low_alarm_enable: Option<bool>,
    pub created_at: OffsetDateTime,
}

// Implement the FromRow trait for Config.
// This allows us to convert a sqlx::Row into a Config struct.
// It is used in the sqlx::query_as function (not the sqlx::query_as! micro).
impl FromRow<'_, PgRow> for Config {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::error::Error> {
        let id = row.get("id");
        let device_id = row.get("device_id");
        let temperature_low_alarm_enable = row.get("temperature_low_alarm_enable");
        let created_at = row.get("created_at");

        let control_temperature: Option<BigDecimal> = row.get("control_temperature");
        let control_temperature = match control_temperature {
            Some(val) => Some(HundredthTemperatureCelcius::from(val)),
            None => None,
        };

        Ok(Self {
            id: id,
            device_id: device_id,
            control_temperature: control_temperature,
            temperature_low_alarm_enable: temperature_low_alarm_enable,
            created_at: created_at,
        })
    }
}