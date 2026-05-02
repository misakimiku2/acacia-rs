use axum::http::HeaderMap;

use super::base::{AccessRequirement, ApiResponse, ApiResult, check_auth};

pub async fn handle_hello(headers: HeaderMap) -> ApiResult<String> {
    check_auth(&headers, AccessRequirement::Public)?;
    Ok(ApiResponse::as_success("Hello acacia-rs.".to_string()))
}
