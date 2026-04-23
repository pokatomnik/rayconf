use crate::v2parser::entities::outbound_settings::OutboundSettings;
use crate::v2parser::entities::raw_data::RawData;
use crate::v2parser::entities::trojan_outbound_settings::TrojanOutboundSettings;
use crate::v2parser::entities::trojan_server_object::TrojanServerObject;

pub mod data;
mod models;

pub fn create_outbound_settings(data: &RawData) -> OutboundSettings {
    return OutboundSettings::Trojan(TrojanOutboundSettings {
        servers: vec![TrojanServerObject {
            address: data.address.clone(),
            port: data.port,
            password: data.uuid.clone(),
            level: Some(0),
        }],
    });
}
