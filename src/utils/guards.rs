use axum::{
    headers::{authorization::Bearer, Authorization, HeaderMapExt},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use super::{api_error::APIError, jwt::decode_jwt};

pub async fn guard<T>(mut req: Request<T>, next: Next<T>) -> Result<Response, APIError> {
    let token = req
        .headers()
        .typed_get::<Authorization<Bearer>>()
        .ok_or(APIError {
            message: "No Auth Token found".to_owned(),
            status_code: StatusCode::BAD_REQUEST,
            error_code: Some(99),
        })?
        .token()
        .to_owned();

    let claim = decode_jwt(token)
        .map_err(|_| APIError {
            message: "Token not Valid".to_owned(),
            status_code: StatusCode::UNAUTHORIZED,
            error_code: Some(123),
        })?
        .claims;

    let db = req.extensions().get::<DatabaseConnection>().unwrap();

    let identity = entity::user::Entity::find()
        .filter(entity::user::Column::Email.eq(claim.email.to_lowercase()))
        .one(db)
        .await
        .map_err(|err| APIError {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(50),
        })?
        .ok_or(APIError {
            message: "User Not Found".to_owned(),
            status_code: StatusCode::UNAUTHORIZED,
            error_code: Some(41),
        })?;

    req.extensions_mut().insert(identity);
    Ok(next.run(req).await)
}
