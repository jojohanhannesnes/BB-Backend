use self::api_error::APIError;

pub mod api_error;
pub mod api_success;

pub type ResultAPI<T> = core::result::Result<T, APIError>; // API Level Error, for Nested error use From inner?
