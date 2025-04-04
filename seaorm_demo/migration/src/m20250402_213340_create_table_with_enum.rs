use sea_orm_migration::{prelude::{extension::postgres::Type, *}, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        // There are two ways to create enum type
        // 1. Create enum type from ActiveEnum
        // 2. Create enum type from TYPE statement
        // see https://www.sea-ql.org/SeaORM/docs/generate-entity/enumeration/
        //let schema = Schema::new(DbBackend::Postgres);
        // manager.create_type(
        //     schema
        //         .create_enum_from_active_enum::<DeviceType>()
        //     ).await?;
        manager
            .create_type(
                Type::create()
                    .as_enum((Device::Schema, DeviceType::EnumName))
                    .values([
                        DeviceType::NGC20,
                        DeviceType::ELEXANT5010I,
                        DeviceType::ELEXANT40X0I,
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table((Device::Schema, Device::Table))
                    .col(pk_auto(Device::Id))
                    .col(
                        ColumnDef::new(Device::DeviceType)
                            .custom(Alias::new("kres.device_type"))
                            .not_null(),
                    )
                    .col(string(Device::Tag))
                    .col(string_null(Device::Vendor))
                    .col(timestamp_with_time_zone(Device::CreatedAt).default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table((Device::Schema, Device::Table)).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name((Device::Schema, DeviceType::EnumName)).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Device {
    Table,
    #[sea_orm(iden = "kres")]
    Schema,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "device_type")]
    DeviceType,
    Tag,
    Vendor,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
}

#[derive(DeriveIden)]
pub enum DeviceType {
    #[sea_orm(iden = "device_type")]
    EnumName,
    #[sea_orm(string_value = "ngc20")]
    NGC20,
    #[sea_orm(string_value = "elexant5010i")]
    ELEXANT5010I,
    #[sea_orm(string_value = "elexant40x0i")]
    ELEXANT40X0I,
}
