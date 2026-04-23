use crate::v2parser::entities::shadowsocks_server_object::ShadowSocksServerObject;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct ShadowSocksOutboundSettings {
    #[serde(rename = "servers")]
    pub servers: Vec<ShadowSocksServerObject>,
}
