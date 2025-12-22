use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use reqwest::{
    Client,
    header::{HeaderMap, HeaderValue},
};

pub enum IPtype {
    IPv4,
    IPv6,
}

pub fn client(ipt: IPtype) -> anyhow::Result<Client> {
    let mut headers = HeaderMap::new();
    let addr = match ipt {
        IPtype::IPv4 => Ipv4Addr::UNSPECIFIED.into(),
        IPtype::IPv6 => Ipv6Addr::UNSPECIFIED.into(),
    };
    headers.append("User-Agent", HeaderValue::from_static("curl/8.17.0"));
    Ok(reqwest::Client::builder()
        .local_address::<IpAddr>(addr)
        .default_headers(headers)
        .build()?)
}
