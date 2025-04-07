use sea_orm_migration::{prelude::*, sea_orm::ActiveEnum};
use entities::sea_orm_active_enums::DeviceType;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let insert = Query::insert()
            .into_table((Device::Schema, Device::Table))
            .columns([Device::DeviceType, Device::Tag, Device::Vendor])
            .values_panic([
                DeviceType::Ngc20.as_enum(DeviceType::name()),
                "NGC20".into(),
                "nVent".into(),
            ])
            .values_panic([
                DeviceType::Elexant5010i.as_enum(DeviceType::name()),
                "ELEXANT5010I".into(),
                "Chemelex".into(),
            ])
            .values_panic([
                DeviceType::Elexant40x0i.as_enum(DeviceType::name()),
                "ELEXANT40X0I".into(),
                "Chemelex".into(),
            ])
            .to_owned();

        manager
            .exec_stmt(insert)
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let delete = Query::delete()
            .from_table((Device::Schema, Device::Table))
            .to_owned();
        manager
            .exec_stmt(delete)
            .await
    }
}

#[derive(DeriveIden)]
enum Device {
    Table,
    #[sea_orm(iden = "kres")]
    Schema,
    #[sea_orm(iden = "device_type")]
    DeviceType,
    Tag,
    Vendor,
}
