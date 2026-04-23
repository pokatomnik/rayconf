use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WSSettings {
    #[serde(rename = "path")]
    pub path: Option<String>,

    #[serde(rename = "Host")]
    pub host: Option<String>,

    #[serde(rename = "acceptProxyProtocol")]
    pub accept_proxy_protocol: Option<bool>,
}
