use std::str::FromStr;

use trust_dns_client::client::{Client, SyncClient, AsyncClient};
use trust_dns_client::op::DnsResponse;
use trust_dns_client::rr::{DNSClass, Name, RData, Record, RecordType};
use trust_dns_client::udp::UdpClientConnection;

const STARLINK_ROOT_DOMAIN_NAME: &str = "starlinkisp.net";

pub fn resolve_ptr(hostname: String) -> Vec<String> {
    let conn = UdpClientConnection::new("1.1.1.1:53".parse().unwrap()).unwrap();
    let (client, bg) = AsyncClient::connect(conn.into()).await.unwrap();
    let name = Name::from_str(&hostname).unwrap();
    let response = client.query(&name, DNSClass::IN, RecordType::PTR).unwrap();
    let answers = response.answers();

    let mut result = Vec::new();
    for answer in answers {
        if let Some(RData::PTR(ref name)) = answer.data() {
            result.push(name.to_string());
        }
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
