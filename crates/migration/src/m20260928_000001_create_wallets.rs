use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Wallets::Table)
                    .col(pk_uuid(Wallets::Id).default(Expr::cust("gen_random_uuid()")))
                    .col(string_len(Wallets::Name, 64))
                    .col(string_len_null(Wallets::Address, 42))
                    .col(string_len(Wallets::Status, 16).default("generating"))
                    .col(
                        timestamp_with_time_zone(Wallets::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Wallets {
    Table,
    Id,
    Name,
    Address,
    Status,
    CreatedAt,
}
