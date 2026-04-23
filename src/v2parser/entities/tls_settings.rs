use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct TLSSettings {
    #[serde(rename = "alpn")]
    pub alpn: Option<Vec<String>>,

    #[serde(rename = "allowInsecure")]
    pub allow_insecure: bool,

    #[serde(rename = "serverName")]
    pub server_name: Option<String>,

    #[serde(rename = "enableSessionResumption")]
    pub enable_session_resumption: Option<bool>,

    #[serde(rename = "disableSystemRoot")]
    pub disable_system_root: Option<bool>,

    #[serde(rename = "minVersion")]
    pub min_version: Option<String>,

    #[serde(rename = "maxVersion")]
    pub max_version: Option<String>,

    #[serde(rename = "cipherSuites")]
    pub cipher_suites: Option<String>,

    #[serde(rename = "preferServerCipherSuites")]
    pub prefer_server_cipher_suites: Option<bool>,

    #[serde(rename = "fingerprint")]
    pub fingerprint: Option<String>,

    #[serde(rename = "rejectUnknownSni")]
    pub reject_unknown_sni: Option<bool>,
}
