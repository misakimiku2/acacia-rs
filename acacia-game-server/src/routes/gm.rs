use axum::{Router, routing::post};

mod gm_script;

pub fn routes() -> Router {
    Router::new().route("/gm/script", post(gm_script::handle_gm_script))
}
