use crate::v2parser::entities::outbound_settings::OutboundSettings;
use crate::v2parser::entities::raw_data::RawData;
use crate::v2parser::entities::socks_outbound_settings::SocksOutboundSettings;
use crate::v2parser::entities::socks_server_object::SocksServerObject;
use crate::v2parser::entities::socks_user::SocksUser;

pub mod data;
mod models;

pub fn create_outbound_settings(data: &RawData) -> OutboundSettings {
    return OutboundSettings::Socks(SocksOutboundSettings {
        servers: vec![SocksServerObject {
            users: match (&data.username, &data.uuid) {
                (Some(username), Some(uuid)) => Some(vec![SocksUser {
                    user: Some(username.clone()),
                    pass: Some(uuid.clone()),
                }]),
                _ => None,
            },
            address: data.address.clone(),
            port: data.port,
            level: Some(0),
        }],
    });
}
