use axum::{
    headers::{authorization::Bearer, Authorization, HeaderMapExt},
    http::Request,
    middleware::Next,
    response::Response,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use super::{
    jwt::decode_jwt,
    mapper::api_error::{APIError, AppError},
};

pub async fn guard<T>(mut req: Request<T>, next: Next<T>) -> Result<Response, APIError> {
    let token = req
        .headers()
        .typed_get::<Authorization<Bearer>>()
        .ok_or(APIError::new(
            AppError::AuthTokenNotFound,
            "No Auth Token found",
        ))?
        .token()
        .to_owned();

    let claim = decode_jwt(token)
        .map_err(|err| APIError::new(AppError::AuthTokenError, err))?
        .claims;

    let db = req.extensions().get::<DatabaseConnection>().unwrap();

    let identity = entity::user::Entity::find()
        .filter(entity::user::Column::Email.eq(claim.email.to_lowercase()))
        .one(db)
        .await
        .map_err(|err| APIError::new(AppError::DbError, err.to_string()))?
        .ok_or(APIError::new(
            AppError::UserNotFound,
            "User not found".to_owned(),
        ))?;

    req.extensions_mut().insert(identity);
    Ok(next.run(req).await)
}
