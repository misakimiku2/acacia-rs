use axum::{Json, http::HeaderMap};

use super::base::*;
use acacia_database::{models::PlayerConfig, repositories::player_config::PlayerConfigRepository};

#[derive(Debug, serde::Deserialize)]
pub struct AccountCreatePlayload {
    pid: u64,
}

pub async fn handle_account_create(headers: HeaderMap, Json(payload): Json<AccountCreatePlayload>) -> ApiResult<Empty> {
    check_auth(&headers, AccessRequirement::Admin)?;
    let config_database = PlayerConfigRepository::new(payload.pid);
    if let Some(_) = config_database.find_one().await.internal()? {
        return Err(ApiResponse::as_bad_request(format!("The specified pid {} already exists.", payload.pid)));
    }
    let config = PlayerConfig {
        pid: payload.pid,
        token: payload.pid.to_string(),
        ..Default::default()
    };
    config_database.update_one(config).await.internal()?;
    Ok(ApiResponse::as_success(None))
}
