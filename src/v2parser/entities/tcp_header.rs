use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct TCPHeader {
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}
