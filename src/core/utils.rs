use std::env;

use super::AppEnv;

pub fn is_prod<T>(_: T) -> bool {
    env::var("ENV")
        .unwrap_or(AppEnv::Dev.to_string())
        .parse::<AppEnv>()
        .unwrap_or(AppEnv::Dev)
        .is_prod()
}

pub fn should_add_openapi_routes() -> bool {
    !is_prod(())
}
