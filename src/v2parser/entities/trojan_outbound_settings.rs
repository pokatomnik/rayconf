use serde::{Deserialize, Serialize};
use crate::v2parser::entities::trojan_server_object::TrojanServerObject;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct TrojanOutboundSettings {
    #[serde(rename = "servers")]
    pub servers: Vec<TrojanServerObject>,
}
