use serde::{Deserialize, Serialize};
use crate::v2parser::entities::outbound_settings::OutboundSettings;
use crate::v2parser::entities::stream_settings::StreamSettings;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Outbound {
    #[serde(rename = "settings")]
    pub settings: OutboundSettings,

    #[serde(rename = "streamSettings")]
    pub stream_settings: StreamSettings,

    #[serde(rename = "protocol")]
    pub protocol: String,

    #[serde(rename = "tag")]
    pub tag: String,
}
