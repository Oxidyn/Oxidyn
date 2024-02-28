use serde::Deserialize;
use super::DnsProviders;

#[derive(Debug, Deserialize, Clone)]
pub struct DuckDNSConfig {
    pub token: String,
    pub domain: String,
}

pub struct DuckDNSProvider {
    config: DuckDNSConfig,
}

impl DuckDNSProvider {
    // Constructor for DuckDNSProvider
    pub fn new(config: &DuckDNSConfig) -> Self {
        DuckDNSProvider { config: config.clone() }
    }
}

impl DnsProviders for DuckDNSProvider {
    fn update_ip(&self, ip: &str) {
        // Implement DuckDNS specific logic to update IP
        println!("Updating IP address on DuckDNS...");
        println!("Token: {}", self.config.token);
        println!("Domain: {}", self.config.domain);
        println!("New IP: {}", ip);
    }
}
