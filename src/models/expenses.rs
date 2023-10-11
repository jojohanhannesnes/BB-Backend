use entity::{expenses::Model, expenses_categories::Model as ExpCategories};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Serialize, Deserialize)]
pub struct ExpensesCategoriesModel {
    pub name: String,
}
#[derive(Serialize, Deserialize)]
pub struct ExpensesModel {
    pub amount: i64,
    pub category: ExpensesCategoriesModel,
    #[serde(skip_serializing)]
    pub user_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct CreateExpensesModel {
    pub amount: i64,
    pub category_id: i32,
}

impl From<(Model, Option<ExpCategories>)> for ExpensesModel {
    fn from(value: (Model, Option<ExpCategories>)) -> Self {
        ExpensesModel {
            amount: value.0.amount,
            category: match value.1 {
                Some(_) => ExpensesCategoriesModel {
                    name: value.1.unwrap().name,
                },
                None => panic!("cannot join categories from expenses"),
            },
            user_id: value.0.user_id,
        }
    }
}
