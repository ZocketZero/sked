//! This module provides a helper function to create a `reqwest::Client`
//! that is configured to use either IPv4 or IPv6 for its local address.

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use reqwest::{
    Client,
    header::{HeaderMap, HeaderValue},
};

/// Enum to specify the desired IP version for the request client.
pub enum IPtype {
    IPv4,
    IPv6,
}

/// Creates a `reqwest::Client` configured to bind to a specific IP protocol version.
/// This allows forcing requests over IPv4 or IPv6.
pub fn client(ipt: IPtype) -> anyhow::Result<Client> {
    let mut headers = HeaderMap::new();
    // Set the local address to an unspecified address of the chosen IP type.
    // This hints to the OS which IP stack to use.
    let addr = match ipt {
        IPtype::IPv4 => Ipv4Addr::UNSPECIFIED.into(),
        IPtype::IPv6 => Ipv6Addr::UNSPECIFIED.into(),
    };
    // Spoof the User-Agent to mimic curl.
    headers.append("User-Agent", HeaderValue::from_static("curl/8.17.0"));
    Ok(reqwest::Client::builder()
        .local_address::<IpAddr>(addr)
        .default_headers(headers)
        .build()?)
}
