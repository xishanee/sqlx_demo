pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_schema;
mod m20250402_213340_create_table_with_enum;
mod m20250404_164806_populate_device_data;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_schema::Migration),
            Box::new(m20250402_213340_create_table_with_enum::Migration),
            Box::new(m20250404_164806_populate_device_data::Migration),
        ]
    }
}
