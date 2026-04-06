use axum::{Json, Router, http::StatusCode, routing::post};
use tracing::debug;

use crate::base::config::CONFIG;
use crate::base::conn::CONNS;
use crate::gm::gm_script::gm_script::GMScript;

pub async fn listen_gm_server() {
    let addr = format!("{}:{}", CONFIG.host, 1234);
    let app = Router::new().route("/gm/script", post(handle_gm_script));
    let listener = tokio::net::TcpListener::bind(&addr).await.expect("GMServer failed to bind portFailed to bind GM port");
    tracing::info!("GMServer listening on {}", addr);
    if let Err(e) = axum::serve(listener, app).await {
        tracing::error!("GMServer server error: {}", e);
    }
}

#[derive(Debug, serde::Deserialize)]
struct GmScriptPayload {
    pid: u64,
    // target: i32,
    code: String,
}

async fn handle_gm_script(Json(payload): Json<GmScriptPayload>) -> (StatusCode, &'static str) {
    if let Some(conn) = CONNS.get_by_uid(payload.pid) {
        if let Some(gm_script) = GMScript::from_code(&payload.code) {
            debug!("GMScript: {:?} {:?}", conn.uid(), gm_script);
            gm_script.run(conn).await.expect("Failed to run GMScript");
        }
    }
    (StatusCode::OK, "success")
}
