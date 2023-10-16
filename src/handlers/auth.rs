use axum::{
    http::{Method, StatusCode, Uri},
    Extension, Json,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, Set,
};

use crate::{
    models::user::{
        CreateUserModel, CreateUserResponseModel, LoginUserModel, LoginUserResponseModel,
    },
    utils::{
        jwt::encode_jwt,
        log::log_request,
        mapper::{
            api_error::APIError,
            api_success::{APISuccess, AppSuccess},
            ResultAPI,
        },
    },
};

pub async fn create_user(
    Extension(db): Extension<DatabaseConnection>,
    Json(user_data): Json<CreateUserModel>,
) -> ResultAPI<APISuccess<CreateUserResponseModel>> {
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
        password: Set(hash(user_data.password, DEFAULT_COST).unwrap()),
        balance: Set(500),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };
    let created_user = user_model.insert(&db).await.map_err(|err| APIError {
        message: err.to_string(),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        error_code: Some(50),
    })?;

    let result = APISuccess::new(
        AppSuccess::UserCreated,
        "User Created",
        CreateUserResponseModel {
            email: user_data.email,
            balance: created_user.balance,
            created_at: created_user.created_at,
        },
    );

    Ok(result)
}

pub async fn login_user(
    uri: Uri,
    method: Method,
    Extension(db): Extension<DatabaseConnection>,
    Json(user_data): Json<LoginUserModel>,
) -> ResultAPI<APISuccess<LoginUserResponseModel>> {
    log_request("Login User", &uri, &method, None, &user_data);
    let user = entity::user::Entity::find()
        .filter(Condition::all().add(entity::user::Column::Email.eq(user_data.email)))
        .one(&db)
        .await
        .map_err(|err| APIError {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(50),
        })?
        .ok_or(APIError {
            message: "User Not Found".to_owned(),
            status_code: StatusCode::NOT_FOUND,
            error_code: Some(44),
        })?;

    let password_verify = verify(user_data.password, &user.password).map_err(|err| APIError {
        message: err.to_string(),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        error_code: Some(20),
    })?;

    if !password_verify {
        return Err(APIError {
            message: "Wrong Password".to_owned(),
            status_code: StatusCode::BAD_REQUEST,
            error_code: Some(40),
        });
    }
    let token = encode_jwt(user.email).map_err(|_| APIError {
        message: "Cannot encode JWT".to_owned(),
        status_code: StatusCode::BAD_REQUEST,
        error_code: Some(124),
    })?;
    let result = APISuccess::new(
        AppSuccess::UserLoggedIn,
        "User success login",
        LoginUserResponseModel { token },
    );
    log_request("Login user success", &uri, &method, Some(user.id), &result);
    Ok(result)
}
