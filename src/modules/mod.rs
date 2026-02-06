//! This module serves as the central hub for all feature-specific modules in `sked`.
//! It defines constants and conditionally compiles and exports modules based on feature flags.

/// `URL_IPV4`: The URL used to fetch the public IPv4 address.
pub const URL_IPV4: &str = "https://ipinfo.io/ip";
/// `URL_IPV6`: The URL used to fetch the public IPv6 address.
pub const URL_IPV6: &str = "https://ifconfig.co";

// Conditionally compile and export the `brute_path` module.
#[cfg(feature = "brute-path")]
mod brute_path;
#[cfg(feature = "brute-path")]
pub use brute_path::*;

// Conditionally compile and export the `public_ip` module.
#[cfg(feature = "public")]
mod public_ip;
#[cfg(feature = "public")]
pub use public_ip::*;
