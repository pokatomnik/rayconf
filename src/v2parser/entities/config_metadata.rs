use serde::{Deserialize, Serialize};

// TODO Decide later
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(unused)]
pub(crate) struct ConfigMetaData {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "protocol")]
    pub protocol: String,

    #[serde(rename = "host")]
    pub host: Option<String>,

    #[serde(rename = "address")]
    pub address: Option<String>,

    #[serde(rename = "port")]
    pub port: Option<u16>,
}
