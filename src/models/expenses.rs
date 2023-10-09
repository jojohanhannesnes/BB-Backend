use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct ExpensesModel {
    pub amount: String,
    pub category_id: i32,
    pub user_id: Uuid,
}
#[derive(Serialize, Deserialize)]
pub struct CreateExpensesModel {
    pub amount: i64,
    pub category_id: i32,
}
