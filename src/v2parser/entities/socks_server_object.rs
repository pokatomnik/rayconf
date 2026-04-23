use serde::{Deserialize, Serialize};
use crate::v2parser::entities::socks_user::SocksUser;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct SocksServerObject {
    #[serde(rename = "address")]
    pub address: Option<String>,

    #[serde(rename = "port")]
    pub port: Option<u16>,

    #[serde(rename = "level")]
    pub level: Option<u8>,

    #[serde(rename = "users")]
    pub users: Option<Vec<SocksUser>>,
}
