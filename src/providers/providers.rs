// providers/mod.rs
pub mod cloudflare;
pub mod duckdns;
// Add similar lines for each .rs file you want to include

pub trait DnsProviders {
 fn update_ip(&self, ip: &str);
}