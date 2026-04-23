use serde::{Deserialize, Serialize};
use crate::v2parser::entities::shadowsocks_outbound_settings::ShadowSocksOutboundSettings;
use crate::v2parser::entities::socks_outbound_settings::SocksOutboundSettings;
use crate::v2parser::entities::trojan_outbound_settings::TrojanOutboundSettings;
use crate::v2parser::entities::vless_outbound_settings::VLessOutboundSettings;
use crate::v2parser::entities::vmess_outbound_settings::VMessOutboundSettings;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OutboundSettings {
    Vless(VLessOutboundSettings),
    Vmess(VMessOutboundSettings),
    Trojan(TrojanOutboundSettings),
    ShadowSocks(ShadowSocksOutboundSettings),
    Socks(SocksOutboundSettings),
}
