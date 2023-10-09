use axum::{http::StatusCode, Extension, Json};
// use axum_macros::debug_handler;
use entity::user::Model;
use sea_orm::{ActiveModelTrait, DatabaseConnection, IntoActiveModel, IntoActiveValue, Set};

use crate::{models::expenses::CreateExpensesModel, utils::api_error::APIError};

// #[debug_handler]
pub async fn create_expenses(
    Extension(db): Extension<DatabaseConnection>,
    Extension(identity): Extension<Model>,
    Json(create_data): Json<CreateExpensesModel>,
) -> Result<(), APIError> {
    let balance = identity.balance - create_data.amount;
    let user_id = identity.id;
    let mut user = identity.into_active_model();
    user.balance = Set(balance);
    let _ = user.update(&db).await.unwrap();

    // .map_err(|_| APIError {
    //     message: "Failed to update user balance".to_owned(),
    //     status_code: StatusCode::INTERNAL_SERVER_ERROR,
    //     error_code: Some(19),
    // });
    entity::expenses::ActiveModel {
        amount: create_data.amount.into_active_value(),
        category_id: create_data.category_id.into_active_value(),
        user_id: user_id.into_active_value(),
        ..Default::default()
    }
    .insert(&db)
    .await
    .map_err(|_| APIError {
        message: "Failed to insert Expenses".to_owned(),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        error_code: Some(11),
    })?;
    Ok(())
}

// pub async fn get_category() {}
