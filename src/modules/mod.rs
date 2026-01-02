mod brute_path;
mod public_ip;

pub const URL_IPV4: &str = "https://ipinfo.io/ip";
pub const URL_IPV6: &str = "https://ifconfig.co";

pub use brute_path::*;
pub use public_ip::*;
