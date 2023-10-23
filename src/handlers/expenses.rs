use axum::{extract::State, Extension, Json};
// use axum_macros::debug_handler;
use entity::user::Model;
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, IntoActiveValue, Set};

use crate::{
    models::expenses::CreateExpensesModel,
    routes::AppState,
    utils::mapper::{
        api_error::{APIError, AppError},
        api_success::{APISuccess, AppSuccess},
        ResultAPI,
    },
};

// #[debug_handler]
pub async fn create_expenses(
    State(AppState { db }): State<AppState>,
    Extension(identity): Extension<Model>,
    Json(create_data): Json<CreateExpensesModel>,
) -> ResultAPI<()> {
    let balance = identity.balance - create_data.amount;
    let user_id = identity.id;
    let mut user = identity.into_active_model();
    user.balance = Set(balance);
    let _ = user.update(&db).await.unwrap();
    entity::expenses::ActiveModel {
        amount: create_data.amount.into_active_value(),
        category_id: create_data.category_id.into_active_value(),
        user_id: user_id.into_active_value(),
        ..Default::default()
    }
    .insert(&db)
    .await
    .map_err(|err| APIError::new(AppError::DbError, err.to_string()))?;
    Ok(())
}

pub async fn get_category(
    State(AppState { db }): State<AppState>,
) -> ResultAPI<APISuccess<Vec<String>>> {
    let categories: Vec<String> = entity::expenses_categories::Entity::find()
        .all(&db)
        .await
        .map_err(|err| APIError::new(AppError::DbError, err.to_string()))?
        .into_iter()
        .map(|category| category.name)
        .collect();

    Ok(APISuccess::new(
        AppSuccess::SuccessGetList,
        "List of categories",
        categories,
    ))
}
