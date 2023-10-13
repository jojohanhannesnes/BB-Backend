use axum::{
    http::{Method, StatusCode, Uri},
    Extension, Json,
};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, Set,
};

use crate::{
    models::user::{CreateUserModel, LoginUserModel, LoginUserResponseModel},
    utils::{
        api_error::{APIError, APISuccess, ResultCustom},
        jwt::encode_jwt,
        log::log_request,
    },
};

pub async fn create_user(
    Extension(db): Extension<DatabaseConnection>,
    Json(user_data): Json<CreateUserModel>,
) -> Result<(), APIError> {
    let user = entity::user::Entity::find()
        .filter(entity::user::Column::Email.eq(user_data.email.clone()))
        .one(&db)
        .await
        .map_err(|err| APIError {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(50),
        })?;

    if user.is_some() {
        return Err(APIError {
            message: "User Exist".to_owned(),
            status_code: StatusCode::CONFLICT,
            error_code: Some(40),
        });
    }

    let user_model = entity::user::ActiveModel {
        name: Set(user_data.name.to_owned()),
        email: Set(user_data.email.to_owned()),
        password: Set(user_data.password.to_owned()),
        balance: Set(500),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };
    user_model.insert(&db).await.map_err(|err| APIError {
        message: err.to_string(),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        error_code: Some(50),
    })?;
    Ok(())
}

pub async fn login_user(
    uri: Uri,
    method: Method,
    Extension(db): Extension<DatabaseConnection>,
    Json(user_data): Json<LoginUserModel>,
) -> ResultCustom<APISuccess<LoginUserResponseModel>> {
    log_request("Login User", &uri, &method, None, &user_data);
    let user = entity::user::Entity::find()
        .filter(
            Condition::all()
                .add(entity::user::Column::Email.eq(user_data.email))
                .add(entity::user::Column::Password.eq(user_data.password)),
        )
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

    let token = encode_jwt(user.email).map_err(|_| APIError {
        message: "Cannot encode JWT".to_owned(),
        status_code: StatusCode::BAD_REQUEST,
        error_code: Some(124),
    })?;
    let result = APISuccess::new("User success login", LoginUserResponseModel { token });
    log_request("Login user success", &uri, &method, Some(user.id), &result);
    Ok(result)
}
