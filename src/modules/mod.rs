pub const URL_IPV4: &str = "https://ipinfo.io/ip";
pub const URL_IPV6: &str = "https://ifconfig.co";

#[cfg(feature = "brute-path")]
mod brute_path;
#[cfg(feature = "brute-path")]
pub use brute_path::*;

#[cfg(feature = "public")]
mod public_ip;
#[cfg(feature = "public")]
pub use public_ip::*;
