use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref DATABASE_URL: String = env_var("DATABASE_URL");
    pub static ref TOKEN: String = env_var("TOKEN");
}

fn env_var(key: &str) -> String {
    dotenv::dotenv().ok();
    env::var(key).unwrap_or_else(|_| panic!("Environment variable {} not found", key))
}
