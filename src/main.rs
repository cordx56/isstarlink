mod checker;
mod index_html;

use axum::{
    extract::Query,
    http::StatusCode,
    response::IntoResponse,
    response::{Html, Response},
    routing::get,
    Json, Router, Server,
};
use axum_client_ip::{SecureClientIp, SecureClientIpSource};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Deserialize)]
struct QueryParam {
    addr: Option<String>,
}

#[derive(Serialize)]
struct ErrorJsonResponse {
    status: bool,
    message: String,
}
#[derive(Serialize)]
struct SuccessJsonResponse {
    status: bool,
    query_addr: String,
    remote_addr: String,
    query_domain: Vec<String>,
    is_starlink: bool,
}

async fn index_html_handler(
    secure_addr: SecureClientIp,
    query_param: Query<QueryParam>,
) -> (StatusCode, Html<String>) {
    let remote_addr = secure_addr.0;
    let query_addr = match &query_param.addr {
        Some(addr_str) => match addr_str.parse() {
            Ok(addr) => addr,
            Err(_) => return (StatusCode::BAD_REQUEST, Html("bad query".to_string())),
        },
        None => remote_addr,
    };
    let ptrs = checker::resolve_ptr(query_addr).await;
    let is_starlink = checker::contains_starlink(&ptrs);

    (
        StatusCode::OK,
        Html(index_html::generate_index_html(
            &query_addr.to_string(),
            &remote_addr.to_string(),
            &ptrs,
            is_starlink,
        )),
    )
}

async fn json_handler(
    secure_addr: SecureClientIp,
    query_param: Query<QueryParam>,
) -> (StatusCode, Response) {
    let remote_addr = secure_addr.0;
    let query_addr = match &query_param.addr {
        Some(addr_str) => match addr_str.parse() {
            Ok(addr) => addr,
            Err(_) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorJsonResponse {
                        status: false,
                        message: "invalid query address".to_string(),
                    })
                    .into_response(),
                )
            }
        },
        None => remote_addr,
    };
    let ptrs = checker::resolve_ptr(query_addr).await;
    let is_starlink = checker::contains_starlink(&ptrs);
    (
        StatusCode::OK,
        Json(SuccessJsonResponse {
            status: true,
            query_addr: query_addr.to_string(),
            remote_addr: remote_addr.to_string(),
            query_domain: ptrs,
            is_starlink: is_starlink,
        })
        .into_response(),
    )
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index_html_handler))
        .route("/json", get(json_handler))
        .layer(SecureClientIpSource::ConnectInfo.into_extension());

    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
