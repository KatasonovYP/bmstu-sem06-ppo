use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Actives::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Actives::ActiveId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Actives::UserId)
                            .integer()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Actives::SecurityId)
                            .string()
                            .not_null()
                    )
                    .col(ColumnDef::new(Actives::BoughtPrice).double().not_null())
                    .col(ColumnDef::new(Actives::Currency).string().not_null())
                    .col(ColumnDef::new(Actives::Count).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user")
                            .from(Actives::Table, Actives::UserId)
                            .to(Users::Table, Users::UserId)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_actives_user_id")
                    .table(Actives::Table)
                    .col(Actives::UserId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Actives::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Actives {
    Table,
    ActiveId,
    UserId,
    SecurityId,
    BoughtPrice,
    Currency,
    Count,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    UserId,
}
