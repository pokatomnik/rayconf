use serde::{Deserialize, Serialize};
use crate::v2parser::entities::tcp_header::TCPHeader;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct TCPSettings {
    #[serde(rename = "header")]
    pub header: Option<TCPHeader>,

    #[serde(rename = "acceptProxyProtocol")]
    pub accept_proxy_protocol: Option<bool>,
}
