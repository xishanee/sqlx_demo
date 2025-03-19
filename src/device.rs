use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(FromRow, Debug, sqlx::Type)]
pub struct EntityId {
    pub id: i32
}

impl From<i32> for EntityId {
    fn from(value: i32) -> Self {
        EntityId { id: value }
    }
}

#[derive(Debug,sqlx::Type)]
#[sqlx(type_name = "kres.device_type", rename_all = "lowercase")]
pub enum DeviceType {
    NGC20,
    ELEXANT5010I,
    ELEXANT40X0I,
}

#[derive(FromRow, Debug)]
pub struct Device {
    pub id: EntityId,
    pub device_type: DeviceType,
    pub tag: String,
    pub vendor: Option<String>,
    pub created_at: OffsetDateTime,
}