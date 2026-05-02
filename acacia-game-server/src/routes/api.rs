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
    Router::new().route("/", get(hello::handle_hello)).route("/account/create", post(account::handle_account_create))
}
