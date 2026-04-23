use crate::v2parser::entities::inbound::Inbound;
use crate::v2parser::entities::inbound_settings::InboundSettings;
use crate::v2parser::entities::sniffing_settings::SniffingSettings;

pub struct InboundGenerationOptions {
    pub socks_port: Option<u16>,
    pub http_port: Option<u16>,
}

pub fn generate_inbound_config(options: InboundGenerationOptions) -> Vec<Inbound> {
    let mut inbounds: Vec<Inbound> = vec![];
    match options.socks_port {
        Some(port) => {
            inbounds.push(generate_socks_inbound(port));
        }
        None => {}
    }

    match options.http_port {
        Some(port) => {
            inbounds.push(generate_http_inbound(port));
        }
        None => {}
    }

    return inbounds;
}

pub fn generate_http_inbound(http_port: u16) -> Inbound {
    return Inbound {
        protocol: String::from("http"),
        port: http_port,
        tag: String::from("http-in"),
        settings: None,
        listen: String::from("127.0.0.1"),
        sniffing: Some(SniffingSettings {
            enabled: Some(true),
            route_only: Some(true),
            metadata_only: Some(false),
            domains_excluded: None,
            dest_override: Some(vec![
                String::from("http"),
                String::from("tls"),
                String::from("quic"),
            ]),
        }),
    };
}

pub fn generate_socks_inbound(socks_port: u16) -> Inbound {
    return Inbound {
        protocol: String::from("socks"),
        port: socks_port,
        tag: String::from("socks-in"),
        listen: String::from("127.0.0.1"),
        settings: Some(InboundSettings { udp: true }),
        sniffing: Some(SniffingSettings {
            enabled: Some(true),
            route_only: Some(true),
            metadata_only: Some(false),
            domains_excluded: None,
            dest_override: Some(vec![
                String::from("http"),
                String::from("tls"),
                String::from("quic"),
            ]),
        }),
    };
}
