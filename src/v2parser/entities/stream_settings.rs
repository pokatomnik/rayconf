use crate::v2parser::entities::tcp_settings::TCPSettings;
use crate::v2parser::entities::tls_settings::TLSSettings;
use crate::v2parser::entities::ws_settings::WSSettings;
use serde::{Deserialize, Serialize};
use crate::v2parser::entities::grpc_settings::GRPCSettings;
use crate::v2parser::entities::kcp_settings::KCPSettings;
use crate::v2parser::entities::quic_settings::QuicSettings;
use crate::v2parser::entities::reality_settings::RealitySettings;
use crate::v2parser::entities::xhttp_settings::XHTTPSettings;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct StreamSettings {
    #[serde(rename = "network")]
    pub network: Option<String>,

    #[serde(rename = "security")]
    pub security: Option<String>,

    #[serde(rename = "tlsSettings")]
    pub tls_settings: Option<TLSSettings>,

    #[serde(rename = "wsSettings")]
    pub ws_settings: Option<WSSettings>,

    #[serde(rename = "tcpSettings")]
    pub tcp_settings: Option<TCPSettings>,

    #[serde(rename = "realitySettings")]
    pub reality_settings: Option<RealitySettings>,

    #[serde(rename = "grpcSettings")]
    pub grpc_settings: Option<GRPCSettings>,

    #[serde(rename = "quicSettings")]
    pub quic_settings: Option<QuicSettings>,

    #[serde(rename = "kcpSettings")]
    pub kcp_settings: Option<KCPSettings>,

    #[serde(rename = "xhttpSettings")]
    pub xhttp_settings: Option<XHTTPSettings>,
}
