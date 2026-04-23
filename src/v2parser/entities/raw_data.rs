use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct RawData {
    #[serde(rename = "remarks")]
    pub remarks: String,

    #[serde(rename = "security")]
    pub security: Option<String>,

    #[serde(rename = "vnext_security")]
    pub vnext_security: Option<String>,

    #[serde(rename = "sni")]
    pub sni: Option<String>,

    #[serde(rename = "fp")]
    pub fp: Option<String>,

    #[serde(rename = "pbk")]
    pub pbk: Option<String>,

    #[serde(rename = "sid")]
    pub sid: Option<String>,

    #[serde(rename = "type")]
    pub r#type: Option<String>,

    #[serde(rename = "flow")]
    pub flow: Option<String>,

    #[serde(rename = "path")]
    pub path: Option<String>,

    #[serde(rename = "encryption")]
    pub encryption: Option<String>,

    #[serde(rename = "header_type")]
    pub header_type: Option<String>,

    #[serde(rename = "host")]
    pub host: Option<String>,

    #[serde(rename = "seed")]
    pub seed: Option<String>,

    #[serde(rename = "quick_security")]
    pub quic_security: Option<String>,

    #[serde(rename = "key")]
    pub r#key: Option<String>,

    #[serde(rename = "mode")]
    pub mode: Option<String>,

    #[serde(rename = "service_name")]
    pub service_name: Option<String>,

    #[serde(rename = "authority")]
    pub authority: Option<String>,

    #[serde(rename = "slpn")]
    pub slpn: Option<String>,

    #[serde(rename = "spx")]
    pub spx: Option<String>,

    #[serde(rename = "alpn")]
    pub alpn: Option<String>,

    #[serde(rename = "extra")]
    pub extra: Option<String>,

    #[serde(rename = "allowInsecure")]
    pub allow_insecure: Option<String>,

    #[serde(rename = "uuid")]
    pub uuid: Option<String>,

    #[serde(rename = "address")]
    pub address: Option<String>,

    #[serde(rename = "port")]
    pub port: Option<u16>,

    #[serde(rename = "server_method")]
    pub server_method: Option<String>,

    #[serde(rename = "username")]
    pub username: Option<String>,
}
