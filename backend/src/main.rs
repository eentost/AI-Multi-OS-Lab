use axum::{routing::get, Json, Router};
use serde::Serialize;
use std::net::SocketAddr;

#[derive(Serialize)]
struct Health {
    status: &'static str,
    service: &'static str,
    version: &'static str,
    implementation_stage: &'static str,
}

async fn health() -> Json<Health> {
    Json(Health {
        status: "ok",
        service: "ai-multi-os-lab-control-plane",
        version: env!("CARGO_PKG_VERSION"),
        implementation_stage: "scaffold",
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/v1/health", get(health));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await.expect("bind API listener");
    axum::serve(listener, app).await.expect("serve API");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn health_reports_scaffold_not_production_readiness() {
        let response = health().await;
        assert_eq!(response.0.status, "ok");
        assert_eq!(response.0.implementation_stage, "scaffold");
    }
}
