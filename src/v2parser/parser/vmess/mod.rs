use crate::v2parser::entities::outbound_settings::OutboundSettings;
use crate::v2parser::entities::raw_data::RawData;
use crate::v2parser::entities::vmess_outbound_settings::VMessOutboundSettings;
use crate::v2parser::entities::vnext_server_object::VNextServerObject;
use crate::v2parser::entities::vnext_user::VNextUser;

pub mod data;
mod models;

pub fn create_outbound_settings(data: &RawData) -> OutboundSettings {
    return OutboundSettings::Vmess(VMessOutboundSettings {
        vnext: vec![VNextServerObject {
            port: data.port,
            address: data.address.clone(),
            users: Some(vec![VNextUser {
                id: data.uuid.clone(),
                flow: data.flow.clone(),
                encryption: Some(data.encryption.clone().unwrap_or(String::from("none"))),
                level: Some(0),
                security: data.vnext_security.clone(),
            }]),
        }],
    });
}
