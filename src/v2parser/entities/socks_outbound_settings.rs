use crate::v2parser::entities::socks_server_object::SocksServerObject;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct SocksOutboundSettings {
    #[serde(rename = "servers")]
    pub servers: Vec<SocksServerObject>,
}
