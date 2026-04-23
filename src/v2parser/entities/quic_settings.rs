use crate::v2parser::entities::non_header_object::NonHeaderObject;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct QuicSettings {
    #[serde(rename = "header")]
    pub header: Option<NonHeaderObject>,

    #[serde(rename = "security")]
    pub security: Option<String>,

    #[serde(rename = "key")]
    pub key: Option<String>,
}
