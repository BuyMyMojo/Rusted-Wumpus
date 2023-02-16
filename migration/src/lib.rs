pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_users;
mod m20230216_062018_pgcrypto;
mod m20230216_062852_qutoe_id_function;
mod m20230216_063045_qutoe;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_users::Migration),
            Box::new(m20230216_062018_pgcrypto::Migration),
            Box::new(m20230216_062852_qutoe_id_function::Migration),
            Box::new(m20230216_063045_qutoe::Migration),
        ]
    }
}
