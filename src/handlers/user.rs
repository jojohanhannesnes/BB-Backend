use axum::{extract::Path, http::StatusCode, Extension, Json};
use entity::user::{ActiveModel, Model};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, Set,
};
use uuid::Uuid;

use crate::{
    models::{
        api::dashboard::DashboardModelResponse,
        expenses::ExpensesModel,
        user::{UpdateUserModel, UserModel},
    },
    utils::api_error::APIError,
};

pub async fn update_user(
    Extension(db): Extension<DatabaseConnection>,
    Path(uuid): Path<Uuid>,
    Json(user_data): Json<UpdateUserModel>,
) -> Result<(), APIError> {
    let mut user: ActiveModel = entity::user::Entity::find()
        .filter(Condition::all().add(entity::user::Column::Id.eq(uuid)))
        .one(&db)
        .await
        .map_err(|err| APIError {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(50),
        })?
        .ok_or(APIError {
            message: "Not Found".to_owned(),
            status_code: StatusCode::NOT_FOUND,
            error_code: Some(44),
        })?
        .into();

    user.name = Set(user_data.name);

    let _ = user.update(&db).await.map_err(|err| APIError {
        message: err.to_string(),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        error_code: Some(50),
    });

    Ok(())
}

pub async fn delete_user(
    Extension(db): Extension<DatabaseConnection>,
    Path(uuid): Path<Uuid>,
) -> Result<(), APIError> {
    let user = entity::user::Entity::find()
        .filter(Condition::all().add(entity::user::Column::Id.eq(uuid)))
        .one(&db)
        .await
        .map_err(|err| APIError {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(50),
        })?
        .ok_or(APIError {
            message: "Not Found".to_owned(),
            status_code: StatusCode::NOT_FOUND,
            error_code: Some(44),
        })?;

    entity::user::Entity::delete_by_id(user.id)
        .exec(&db)
        .await
        .map_err(|err| APIError {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(50),
        })?;

    Ok(())
}

pub async fn list_user(
    Extension(db): Extension<DatabaseConnection>,
) -> Result<Json<Vec<UserModel>>, APIError> {
    let user: Vec<UserModel> = entity::user::Entity::find()
        .all(&db)
        .await
        .map_err(|err| APIError {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(50),
        })?
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

    Ok(Json(user))
}

pub async fn dashboard_user(
    Extension(db): Extension<DatabaseConnection>,
    Extension(identity): Extension<Model>,
) -> Result<Json<DashboardModelResponse>, APIError> {
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
    Ok(Json(result))
}
