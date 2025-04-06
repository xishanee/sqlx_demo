use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let insert = Query::insert()
            .into_table((Device::Schema, Device::Table))
            .columns([Device::DeviceType, Device::Tag, Device::Vendor])
            .values_panic([
                "ngc20".into(),
                "NGC20".into(),
                "NVIDIA".into(),
            ])
            .values_panic([
                "elexant5010i".into(),
                "ELEXANT5010I".into(),
                "NVIDIA".into(),
            ])
            .values_panic([
                "elexant40x0i".into(),
                "ELEXANT40X0I".into(),
                "NVIDIA".into(),
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
