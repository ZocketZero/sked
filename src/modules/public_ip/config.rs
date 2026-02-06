//! This module contains configuration constants for the `public-ip` feature.
//! It defines the API endpoints for fetching IP addresses.

/// The URL to get the public IPv4 address in plain text.
pub const URL_IPV4: &str = "https://ipapi.co/ip";
/// The URL to get the public IPv6 address in plain text.
/// Note: This is the same as URL_IPV4 because ipapi.co returns the correct IP type based on the request.
pub const URL_IPV6: &str = "https://ipapi.co/ip";

/// The URL to get detailed public IPv4 information in JSON format.
pub const URL_IPV4_V: &str = "https://ipapi.co/json";
/// The URL to get detailed public IPv6 information in JSON format.
pub const URL_IPV6_V: &str = "https://ipapi.co/json";
