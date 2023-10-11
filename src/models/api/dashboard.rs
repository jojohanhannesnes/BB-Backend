use serde::{Deserialize, Serialize};

use crate::models::{expenses::ExpensesModel, user::UserModel};

#[derive(Serialize, Deserialize)]
pub struct DashboardModelResponse {
    pub user: UserModel,
    pub expenses: Vec<ExpensesModel>,
}
