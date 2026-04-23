use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct VNextUser {
    #[serde(rename = "id")]
    pub id: Option<String>,

    #[serde(rename = "encryption")]
    pub encryption: Option<String>,

    #[serde(rename = "flow")]
    pub flow: Option<String>,

    #[serde(rename = "level")]
    pub level: Option<u8>,

    #[serde(rename = "security")]
    pub security: Option<String>,
}
