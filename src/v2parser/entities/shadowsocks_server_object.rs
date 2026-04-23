use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct ShadowSocksServerObject {
    #[serde(rename = "address")]
    pub address: Option<String>,

    #[serde(rename = "port")]
    pub port: Option<u16>,

    #[serde(rename = "password")]
    pub password: Option<String>,

    #[serde(rename = "level")]
    pub level: Option<u8>,

    #[serde(rename = "method")]
    pub method: Option<String>,
}
