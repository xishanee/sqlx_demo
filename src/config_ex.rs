use sqlx::FromRow;
use crate::kres_types::HundredthTemperatureCelcius;
use crate::kres_types::Optional;

#[derive(Debug, FromRow)]
pub struct ConfigEx {
    pub device_id:i32,
    pub control_temperature: Optional<HundredthTemperatureCelcius>,
    pub temperature_low_alarm_enable: Option<bool>,
}