use axum::{
    extract::{Path, State},
    Extension, Json,
};
use entity::user::{ActiveModel, Model};
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::{
    models::{
        api::dashboard::DashboardModelResponse,
        expenses::ExpensesModel,
        user::{UpdateUserModel, UserModel},
    },
    routes::AppState,
    utils::mapper::{
        api_error::{APIError, AppError},
        api_success::{APISuccess, AppSuccess},
        ResultAPI,
    },
};

pub async fn update_user(
    State(AppState { db }): State<AppState>,
    Path(uuid): Path<Uuid>,
    Json(user_data): Json<UpdateUserModel>,
) -> ResultAPI<()> {
    let mut user: ActiveModel = entity::user::Entity::find()
        .filter(Condition::all().add(entity::user::Column::Id.eq(uuid)))
        .one(&db)
        .await
        .map_err(|err| APIError::new(AppError::DbError, err.to_string()))?
        .ok_or(APIError::new(AppError::UserNotFound, "User not found"))?
        .into();

    user.name = Set(user_data.name);

    let _ = user
        .update(&db)
        .await
        .map_err(|err| APIError::new(AppError::DbError, err.to_string()));

    Ok(())
}

pub async fn delete_user(
    State(AppState { db }): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> ResultAPI<()> {
    let user = entity::user::Entity::find()
        .filter(Condition::all().add(entity::user::Column::Id.eq(uuid)))
        .one(&db)
        .await
        .map_err(|err| APIError::new(AppError::DbError, err.to_string()))?
        .ok_or(APIError::new(AppError::UserNotFound, "User not found"))?;

    entity::user::Entity::delete_by_id(user.id)
        .exec(&db)
        .await
        .map_err(|err| APIError::new(AppError::DbError, err.to_string()))?;

    Ok(())
}

pub async fn list_user(
    State(AppState { db }): State<AppState>,
) -> ResultAPI<APISuccess<Vec<UserModel>>> {
    let user: Vec<UserModel> = entity::user::Entity::find()
        .all(&db)
        .await
        .map_err(|err| APIError::new(AppError::DbError, err.to_string()))?
        .into_iter()
        .map(|user| UserModel {
            id: user.id,
            name: user.name,
            email: user.email,
            password: user.password,
            balance: user.balance,
            created_at: user.created_at,
        })
        .collect();

    Ok(APISuccess::new(
        AppSuccess::SuccessGetList,
        "List of users",
        user,
    ))
}

pub async fn dashboard_user(
    State(AppState { db }): State<AppState>,
    Extension(identity): Extension<Model>,
) -> ResultAPI<APISuccess<DashboardModelResponse>> {
    let user: UserModel = identity.into();
    let expenses = entity::expenses::Entity::find()
        .find_also_related(entity::expenses_categories::Entity)
        .filter(entity::expenses::Column::UserId.eq(user.id))
        .all(&db)
        .await
        .expect("Error")
        .into_iter()
        .map(ExpensesModel::from)
        .collect();
    let result = DashboardModelResponse { user, expenses };
    Ok(APISuccess::new(
        AppSuccess::SuccessGetList,
        "User Dashboard",
        result,
    ))
}
