use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct RealitySettings {
    #[serde(rename = "fingerprint")]
    pub fingerprint: Option<String>,

    #[serde(rename = "serverName")]
    pub server_name: Option<String>,

    #[serde(rename = "publicKey")]
    pub public_key: Option<String>,

    #[serde(rename = "shortId")]
    pub short_id: Option<String>,

    #[serde(rename = "spiderX")]
    pub spider_x: Option<String>,
}
