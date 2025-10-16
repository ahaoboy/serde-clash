use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub port: u16,
    pub socks_port: u16,
    pub allow_lan: bool,
    pub mode: String,
    pub log_level: String,
    pub external_controller: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns: Option<DnsConfig>,
    #[serde(default)]
    pub proxies: Vec<Proxy>,
    pub proxy_groups: Vec<ProxyGroup>,
    pub rules: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DnsConfig {
    pub enable: bool,
    pub nameserver: Vec<String>,
    pub fallback: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Proxy {
    pub name: String,
    pub server: String,
    pub port: u16,
    #[serde(rename = "type")]
    pub proxy_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cipher: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_fingerprint: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ProxyGroup {
    pub name: String,
    #[serde(rename = "type")]
    pub group_type: String,
    #[serde(default)]
    pub proxies: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tolerance: Option<u32>,
}
