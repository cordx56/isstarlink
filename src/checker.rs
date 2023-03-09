use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    TokioAsyncResolver,
};
use std::net::IpAddr;

const STARLINK_ROOT_DOMAIN_NAME: &str = "starlinkisp.net.";

pub async fn resolve_ptr(addr: IpAddr) -> Vec<String> {
    let resolver = TokioAsyncResolver::tokio(ResolverConfig::cloudflare(), ResolverOpts::default())
        .expect("failed to connect resolver");

    let response = resolver
        .reverse_lookup(addr)
        .await
        .unwrap();

    let mut result = Vec::new();
    for name in response.iter() {
        result.push(name.to_string());
    }
    result
}

pub fn contains_starlink(names: &[String]) -> bool {
    for name in names {
        if name.ends_with(STARLINK_ROOT_DOMAIN_NAME) {
            return true;
        }
    }
    false
}
