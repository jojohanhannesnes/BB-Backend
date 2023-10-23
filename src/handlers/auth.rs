use std::{thread, time::Duration};

use axum::{
    extract::State,
    http::{Method, Uri},
    Json,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter, Set};

use crate::{
    models::user::{
        CreateUserModel, CreateUserResponseModel, LoginUserModel, LoginUserResponseModel,
    },
    routes::AppState,
    utils::{
        jwt::encode_jwt,
        log::log_request,
        mapper::{
            api_error::{APIError, AppError},
            api_success::{APISuccess, AppSuccess},
            ResultAPI,
        },
    },
};

pub async fn create_user(
    State(AppState { db }): State<AppState>,
    Json(user_data): Json<CreateUserModel>,
) -> ResultAPI<APISuccess<CreateUserResponseModel>> {
    let user = entity::user::Entity::find()
        .filter(entity::user::Column::Email.eq(user_data.email.clone()))
        .one(&db)
        .await
        .map_err(|err| APIError::new(AppError::DbError, err.to_string()))?;

    if user.is_some() {
        return Err(APIError::new(
            AppError::UserAlreadyExist,
            "User Already Exist",
        ));
    }

    let user_model = entity::user::ActiveModel {
        name: Set(user_data.name.to_owned()),
        email: Set(user_data.email.to_owned()),
        password: Set(hash(user_data.password, DEFAULT_COST).unwrap()),
        balance: Set(500),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };
    let created_user = user_model
        .insert(&db)
        .await
        .map_err(|err| APIError::new(AppError::DbError, err.to_string()))?;

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
    State(AppState { db }): State<AppState>,
    Json(user_data): Json<LoginUserModel>,
) -> ResultAPI<APISuccess<LoginUserResponseModel>> {
    thread::sleep(Duration::from_secs(5));
    log_request("Login User", &uri, &method, None, &user_data);
    let user = entity::user::Entity::find()
        .filter(Condition::all().add(entity::user::Column::Email.eq(user_data.email)))
        .one(&db)
        .await
        .map_err(|err| APIError::new(AppError::DbError, err.to_string()))?
        .ok_or(APIError::new(AppError::UserNotFound, "User not found"))?;

    let password_verify = verify(user_data.password, &user.password)
        .map_err(|err| APIError::new(AppError::PasswordVerifyError, err.to_string()))?;

    if !password_verify {
        return Err(APIError::new(AppError::InvalidPassword, "Invalid password"));
    }
    let token = encode_jwt(user.email)
        .map_err(|_| APIError::new(AppError::PasswordEncryptionError, "cannot encrypt password"))?;
    let result = APISuccess::new(
        AppSuccess::UserLoggedIn,
        "User success login",
        LoginUserResponseModel { token },
    );
    log_request("Login user success", &uri, &method, Some(user.id), &result);
    Ok(result)
}
