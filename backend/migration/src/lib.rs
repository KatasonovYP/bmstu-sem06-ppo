pub use sea_orm_migration::prelude::*;

mod m20250412_114652_create_users;
mod m20250412_115929_create_actives;
mod m20250412_120843_create_notifications;
mod m20250513_162421_create_sent;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250412_114652_create_users::Migration),
            Box::new(m20250412_115929_create_actives::Migration),
            Box::new(m20250412_120843_create_notifications::Migration),
            Box::new(m20250513_162421_create_sent::Migration),
        ]
    }
}
