use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct Config {
    #[serde(default)]
    pub port: u16,
    #[serde(default)]
    pub mixed_port: u16,
    #[serde(default)]
    pub socks_port: u16,
    #[serde(default)]
    pub allow_lan: bool,
    #[serde(default)]
    pub mode: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub external_controller: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub external_ui: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub secret: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns: Option<DnsConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub proxies: Vec<Proxy>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub proxy_groups: Vec<ProxyGroup>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Deserialize, Serialize)]
pub struct DnsConfig {
    pub enable: bool,
    pub nameserver: Vec<String>,
    #[serde(default)]
    pub fallback: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Deserialize, Serialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Deserialize, Serialize)]
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
