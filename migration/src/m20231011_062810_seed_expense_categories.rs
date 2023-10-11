use sea_orm_migration::prelude::*;

use crate::m20231009_080258_create_expenses_categories::ExpensesCategories;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let insert = Query::insert()
            .into_table(ExpensesCategories::Table)
            .columns([ExpensesCategories::Name])
            .values_panic(["Transport".into()])
            .values_panic(["Food".into()])
            .values_panic(["Internet".into()])
            .values_panic(["Utilities".into()])
            .to_owned();

        manager.exec_stmt(insert).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ExpensesCategories::Table).to_owned())
            .await
    }
}
