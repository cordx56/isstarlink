mod checker;

use axum::{http::StatusCode, routing::get, Json, Router, Server};
use axum_client_ip::{SecureClientIp, SecureClientIpSource};
use serde::Serialize;
use std::net::SocketAddr;

#[derive(Serialize)]
struct JsonResponse {
    status: bool,
    remote_addr: String,
    query_addr: String,
    is_starlink: bool,
}

async fn json(secure_addr: SecureClientIp) -> (StatusCode, Json<JsonResponse>) {
    let ptrs = checker::resolve_ptr(secure_addr.0.to_string());
    let is_starlink = checker::contains_starlink(&ptrs);
    (
        StatusCode::OK,
        Json(JsonResponse {
            status: true,
            remote_addr: secure_addr.0.to_string(),
            query_addr: secure_addr.0.to_string(),
            is_starlink: is_starlink,
        }),
    )
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/json", get(json))
        .layer(SecureClientIpSource::ConnectInfo.into_extension());

    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await;
}
