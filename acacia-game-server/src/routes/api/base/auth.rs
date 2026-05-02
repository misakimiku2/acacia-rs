use super::response::ApiResponse;
use crate::routes::api::base::response::{ApiResponseResult, Empty};
use axum::http::HeaderMap;

const ADMIN_TOKEN: &str = "8vCzZoprtbQALRt1VzrJi60Xw6RTjjUHxLwk5v69f3VuHzgyXZ1xmd3Ydhz072kA"; // Temporary

pub enum AccessRequirement {
    Public,
    Admin,
    User(u64),
}

pub fn check_auth(headers: &HeaderMap, requirement: AccessRequirement) -> Result<(), ApiResponseResult<Empty>> {
    match requirement {
        AccessRequirement::Public => Ok(()),
        AccessRequirement::Admin => {
            let token = headers.get("X-Token").and_then(|v| v.to_str().ok()).ok_or_else(ApiResponse::as_unauthorized)?;
            if token == ADMIN_TOKEN { Ok(()) } else { Err(ApiResponse::as_unauthorized()) }
        }
        AccessRequirement::User(_user_id) => Err(ApiResponse::as_unauthorized()),
    }
}
