pub use sea_orm_migration::prelude::*;

mod m20231009_060548_create_user;
mod m20231009_080258_create_expenses_categories;
mod m20231009_080306_create_expenses;
mod m20231011_062810_seed_expense_categories;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20231009_060548_create_user::Migration),
            Box::new(m20231009_080258_create_expenses_categories::Migration),
            Box::new(m20231009_080306_create_expenses::Migration),
            Box::new(m20231011_062810_seed_expense_categories::Migration),
        ]
    }
}
