use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct XHTTPSettings {
    #[serde(rename = "host")]
    pub host: Option<String>,

    #[serde(rename = "path")]
    pub path: Option<String>,

    #[serde(rename = "mode")]
    pub mode: Option<String>,

    #[serde(rename = "extra")]
    pub extra: Option<serde_json::Value>,
}
