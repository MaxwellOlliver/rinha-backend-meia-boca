pub use sea_orm_migration::prelude::*;

mod m20240522_033905_create_users_table;
mod m20240523_042709_create_wallets_table;
mod m20240523_050236_create_transactions_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240522_033905_create_users_table::Migration),
            Box::new(m20240523_042709_create_wallets_table::Migration),
            Box::new(m20240523_050236_create_transactions_table::Migration),
        ]
    }
}
