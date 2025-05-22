use sea_orm_migration::{
    prelude::*,
    schema::*,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Sent::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Sent::NotificationId)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(timestamp_with_time_zone(Sent::LastMessageTime))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_notification")
                            .from(Sent::Table, Sent::NotificationId)
                            .to(Notifications::Table, Notifications::NotificationId)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_sent_notification_id")
                    .table(Notifications::Table)
                    .col(Sent::NotificationId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Sent::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Notifications {
    Table,
    NotificationId,
}

#[derive(DeriveIden)]
enum Sent {
    Table,
    NotificationId,
    LastMessageTime,
}
