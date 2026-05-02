use axum::{Json, http::HeaderMap};

use super::base::{AccessRequirement, ApiResponse, ApiResponseError, ApiResult, Empty, check_auth};
use acacia_database::{models::PlayerConfig, repositories::player_config::PlayerConfigRepository};

#[derive(Debug, serde::Deserialize)]
pub struct AccountCreatePlayload {
    pid: u64,
}

pub async fn handle_account_create(headers: HeaderMap, Json(payload): Json<AccountCreatePlayload>) -> ApiResult<Empty> {
    check_auth(&headers, AccessRequirement::Admin)?;
    let config_database = PlayerConfigRepository::new(payload.pid);
    let config = PlayerConfig {
        pid: payload.pid,
        token: payload.pid.to_string(),
        ..Default::default()
    };
    config_database.update_one(config).await.internal()?;
    Ok(ApiResponse::as_success(None))
}
