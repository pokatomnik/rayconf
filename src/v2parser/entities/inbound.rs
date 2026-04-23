use serde::{Deserialize, Serialize};
use crate::v2parser::entities::inbound_settings::InboundSettings;
use crate::v2parser::entities::sniffing_settings::SniffingSettings;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Inbound {
    #[serde(rename = "listen")]
    pub listen: String,

    #[serde(rename = "port")]
    pub port: u16,

    #[serde(rename = "protocol")]
    pub protocol: String,

    #[serde(rename = "settings")]
    pub settings: Option<InboundSettings>,

    #[serde(rename = "sniffing")]
    pub sniffing: Option<SniffingSettings>,

    #[serde(rename = "tag")]
    pub tag: String,
}
