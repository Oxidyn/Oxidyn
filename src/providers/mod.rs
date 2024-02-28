// providers/mod.rs

use std::collections::HashMap;

// Import the providers
pub mod cloudflare;
pub mod duckdns;

// Define a trait for all providers
pub trait DnsProviders {
    fn update_ip(&self, ip: &str);
}

// Define a struct to hold information about each provider
pub struct ProviderInfo {
    struct_name: &'static str,
    constructor: fn(&'static str) -> Box<dyn DnsProviders>,
}

// Implement the ProviderInfo struct
impl ProviderInfo {
    pub fn new(struct_name: &'static str, constructor: fn(&'static str) -> Box<dyn DnsProviders>) -> Self {
        Self { struct_name, constructor }
    }

    pub fn struct_name(&self) -> &'static str {
        self.struct_name
    }

    pub fn create_instance(&self, config: &'static str) -> Box<dyn DnsProviders> {
        (self.constructor)(config)
    }
}

// Function to provide access to all available providers
pub fn providers() -> HashMap<&'static str, ProviderInfo> {
    let mut providers = HashMap::new();
    providers.insert("CloudflareProvider", ProviderInfo::new("CloudflareProvider", cloudflare::CloudflareProvider::new));
    providers.insert("DuckDNSProvider", ProviderInfo::new("DuckDNSProvider", duckdns::DuckDNSProvider::new));
    // Add more providers as needed
    providers
}
