use axum::{extract::Path, http::StatusCode, Extension, Json};
use entity::user::ActiveModel;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, Set,
};
use uuid::Uuid;

use crate::{
    models::user::{UpdateUserModel, UserModel},
    utils::api_error::APIError,
};

pub async fn update_user(
    Extension(db): Extension<DatabaseConnection>,
    Path(uuid): Path<Uuid>,
    Json(user_data): Json<UpdateUserModel>,
) -> Result<(), APIError> {
    let mut user: ActiveModel = entity::user::Entity::find()
        .filter(Condition::all().add(entity::user::Column::Uuid.eq(uuid)))
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
        .filter(Condition::all().add(entity::user::Column::Uuid.eq(uuid)))
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
            name: user.name,
            email: user.email,
            password: user.password,
            uuid: user.uuid,
            created_at: user.created_at,
        })
        .collect();

    Ok(Json(user))
}
