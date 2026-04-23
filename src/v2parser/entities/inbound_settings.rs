use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct InboundSettings {
    #[serde(rename = "udp")]
    pub udp: bool,
}
