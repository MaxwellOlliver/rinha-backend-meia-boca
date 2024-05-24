use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Transactions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Transactions::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("uuid_generate_v4()")),
                    )
                    .col(ColumnDef::new(Transactions::UserId).uuid().not_null())
                    .col(
                        ColumnDef::new(Transactions::TransactionType)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Transactions::Amount)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Transactions::RelatedUserId).uuid().null(), // Usuário relacionado em caso de transferência
                    )
                    .col(
                        ColumnDef::new(Transactions::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_transactions_user")
                            .from(Transactions::Table, Transactions::UserId)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Transactions::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Transactions {
    Table,
    Id,
    UserId,
    TransactionType,
    Amount,
    RelatedUserId,
    CreatedAt,
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
}
