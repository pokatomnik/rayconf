use crate::v2parser::entities::outbound_settings::OutboundSettings;
use crate::v2parser::entities::raw_data::RawData;
use crate::v2parser::entities::shadowsocks_outbound_settings::ShadowSocksOutboundSettings;
use crate::v2parser::entities::shadowsocks_server_object::ShadowSocksServerObject;

pub mod data;
mod models;

pub fn create_outbound_settings(data: &RawData) -> OutboundSettings {
    return OutboundSettings::ShadowSocks(ShadowSocksOutboundSettings {
        servers: vec![ShadowSocksServerObject {
            address: data.address.clone(),
            port: data.port,
            password: data.uuid.clone(),
            level: Some(0),
            method: data.server_method.clone(),
        }],
    });
}
