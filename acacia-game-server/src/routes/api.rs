use axum::{
    Router,
    routing::{get, post},
};

mod base {
    mod auth;
    mod response;
    pub use auth::*;
    pub use response::*;
}
mod account;
mod hello;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(hello::handle_hello))
        .route("/account/create", post(account::handle_account_create))
        .route("/account/list", get(account::handle_account_list))
        .fallback(handle_not_found)
}

async fn handle_not_found() -> base::ApiResponseResult<base::Empty> {
    base::ApiResponse::as_not_found()
}
