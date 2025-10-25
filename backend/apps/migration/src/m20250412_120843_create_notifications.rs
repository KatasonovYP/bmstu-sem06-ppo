use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Notifications::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Notifications::NotificationId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Notifications::PortfolioId)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Notifications::ActiveId).integer().not_null())
                    .col(
                        ColumnDef::new(Notifications::LimitUpper)
                            .double()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Notifications::LimitLower)
                            .double()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Notifications::LimitType).string().not_null())
                    .col(
                        ColumnDef::new(Notifications::ResendIntervalSec)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_active")
                            .from(Notifications::Table, Notifications::ActiveId)
                            .to(Actives::Table, Actives::ActiveId)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_notifications_active_id")
                    .table(Notifications::Table)
                    .col(Notifications::ActiveId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_notifications_portfolio_id")
                    .table(Notifications::Table)
                    .col(Notifications::PortfolioId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Notifications::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Notifications {
    Table,
    NotificationId,
    PortfolioId,
    ActiveId,
    LimitUpper,
    LimitLower,
    LimitType,
    ResendIntervalSec,
}

#[derive(DeriveIden)]
enum Actives {
    Table,
    ActiveId,
}
