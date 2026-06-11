use axum::{Json, http::StatusCode};

pub type Empty = Option<()>;
pub type ApiResult<T> = Result<ApiResponseResult<T>, ApiResponseResult<Empty>>;
pub type ApiResponseResult<T> = (StatusCode, Json<ApiResponse<T>>);

#[derive(serde::Serialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: T,
}

impl<T> ApiResponse<T> {
    pub fn as_success(data: T) -> ApiResponseResult<T> {
        (
            StatusCode::OK,
            Json(ApiResponse {
                code: 0,
                message: "success".to_string(),
                data,
            }),
        )
    }
}

impl ApiResponse<Empty> {
    pub fn as_bad_request(message: String) -> ApiResponseResult<Empty> {
        (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse {
                code: 400,
                message: message,
                data: None,
            }),
        )
    }

    pub fn as_internal_server_error() -> ApiResponseResult<Empty> {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                code: 500,
                message: "Internal server error".to_string(),
                data: None,
            }),
        )
    }
    pub fn as_unauthorized() -> ApiResponseResult<Empty> {
        (
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse {
                code: 401,
                message: "Unauthorized".to_string(),
                data: None,
            }),
        )
    }

    pub fn as_not_found() -> ApiResponseResult<Empty> {
        (
            StatusCode::NOT_FOUND,
            Json(ApiResponse {
                code: 404,
                message: "Not found".to_string(),
                data: None,
            }),
        )
    }
}

pub trait ApiResponseError<T> {
    fn internal_error(self, message: &'static str) -> Result<T, ApiResponseResult<Empty>>;
    fn internal(self) -> Result<T, ApiResponseResult<Empty>>;
}

impl<T, E> ApiResponseError<T> for Result<T, E>
where
    E: std::fmt::Debug,
{
    fn internal_error(self, message: &'static str) -> Result<T, ApiResponseResult<Empty>> {
        self.map_err(|err| {
            tracing::error!(?err, message);
            ApiResponse::as_internal_server_error()
        })
    }
    fn internal(self) -> Result<T, ApiResponseResult<Empty>> {
        self.map_err(|err| {
            tracing::error!(?err);
            ApiResponse::as_internal_server_error()
        })
    }
}
