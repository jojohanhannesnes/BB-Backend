use sea_orm_migration::prelude::*;

use crate::{
    m20231009_060548_create_user::User,
    m20231009_080258_create_expenses_categories::ExpensesCategories,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Expenses::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Expenses::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Expenses::Amount).big_integer().not_null())
                    .col(ColumnDef::new(Expenses::CategoryId).integer().not_null())
                    .col(ColumnDef::new(Expenses::UserId).uuid().not_null())
                    .col(
                        ColumnDef::new(Expenses::CreatedAt)
                            .date_time()
                            .unique_key()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-expenses-categories_id")
                            .from(Expenses::Table, Expenses::CategoryId)
                            .to(ExpensesCategories::Table, ExpensesCategories::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user_id")
                            .from(Expenses::Table, Expenses::UserId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Expenses::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Expenses {
    Table,
    Id,
    Amount,
    CategoryId,
    UserId,
    CreatedAt,
}
