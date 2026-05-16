use crate::v2parser::entities::outbound::Outbound;
use crate::v2parser::entities::{inbound::Inbound, log::Log};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Config {
    #[serde(rename = "log")]
    pub log: Option<Log>,

    #[serde(rename = "inbounds")]
    pub inbounds: Vec<Inbound>,

    #[serde(rename = "outbounds")]
    pub outbounds: Vec<Outbound>,
}
