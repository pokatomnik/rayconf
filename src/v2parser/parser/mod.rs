use crate::v2parser::entities::config::Config;
use crate::v2parser::entities::grpc_settings::GRPCSettings;
use crate::v2parser::entities::kcp_settings::KCPSettings;
use crate::v2parser::entities::non_header_object::NonHeaderObject;
use crate::v2parser::entities::outbound::Outbound;
use crate::v2parser::entities::outbound_settings::OutboundSettings;
use crate::v2parser::entities::quic_settings::QuicSettings;
use crate::v2parser::entities::raw_data::RawData;
use crate::v2parser::entities::reality_settings::RealitySettings;
use crate::v2parser::entities::stream_settings::StreamSettings;
use crate::v2parser::entities::tcp_header::TCPHeader;
use crate::v2parser::entities::tcp_settings::TCPSettings;
use crate::v2parser::entities::tls_settings::TLSSettings;
use crate::v2parser::entities::ws_settings::WSSettings;
use crate::v2parser::entities::xhttp_settings::XHTTPSettings;
use crate::v2parser::utils::inbound_generator::{
    generate_inbound_config, InboundGenerationOptions,
};
use crate::v2parser::utils::parse_raw_json;

mod shadow_socks;
mod socks;
mod trojan;
mod uri_identifier;
mod vless;
mod vmess;

// TODO Decide later should this be removed or not
// pub fn get_metadata(uri: &str) -> String {
//     let (protocol, data, _) = get_uri_data(uri);
//     let meta_data = ConfigMetaData {
//         name: data.remarks,
//         host: data.host.clone(),
//         address: data.address.clone(),
//         port: data.port.clone(),
//         protocol,
//     };
//     let serialized = serde_json::to_string_pretty(&meta_data).unwrap();
//     return serialized;
// }

pub fn create_json_config(uri: &str, socks_port: Option<u16>, http_port: Option<u16>) -> String {
    let config = create_config(uri, socks_port, http_port);
    let serialized = serde_json::to_string_pretty(&config).unwrap();
    return serialized;
}

fn create_config(uri: &str, socks_port: Option<u16>, http_port: Option<u16>) -> Config {
    let outbound_object = create_outbound_object(uri);
    let inbound_config = generate_inbound_config(InboundGenerationOptions {
        socks_port,
        http_port,
    });
    let config = Config {
        outbounds: vec![outbound_object],
        inbounds: inbound_config,
    };
    return config;
}

fn create_outbound_object(uri: &str) -> Outbound {
    let (name, data, outbound_settings) = get_uri_data(uri);

    let network_type = data.r#type.clone().unwrap_or(String::from(""));
    let allow_insecure = data.allow_insecure == Some(String::from("true"))
        || data.allow_insecure == Some(String::from("1"));

    let outbound = Outbound {
        protocol: name,
        tag: String::from("proxy"),
        stream_settings: StreamSettings {
            network: data.r#type.clone(),
            security: data.security.clone(),
            tls_settings: if data.security == Some(String::from("tls")) {
                Some(TLSSettings {
                    alpn: data.alpn.map(|alpn| vec![alpn]),
                    reject_unknown_sni: None,
                    enable_session_resumption: None,
                    min_version: None,
                    max_version: None,
                    cipher_suites: None,
                    disable_system_root: None,
                    prefer_server_cipher_suites: None,
                    fingerprint: data.fp.clone(),
                    server_name: data.sni.clone(),
                    allow_insecure: allow_insecure,
                })
            } else {
                None
            },
            ws_settings: if network_type == String::from("ws") {
                Some(WSSettings {
                    host: data.host.clone(),
                    path: data.path.clone(),
                    accept_proxy_protocol: None,
                })
            } else {
                None
            },
            tcp_settings: if network_type == String::from("tcp") {
                Some(TCPSettings {
                    header: Some(TCPHeader {
                        r#type: Some(data.header_type.unwrap_or(String::from("none"))),
                    }),
                    accept_proxy_protocol: None,
                })
            } else {
                None
            },
            reality_settings: if data.security == Some(String::from("reality")) {
                Some(RealitySettings {
                    public_key: data.pbk,
                    server_name: data.sni.clone(),
                    short_id: data.sid,
                    spider_x: Some(String::from("")),
                    fingerprint: data.fp.clone(),
                })
            } else {
                None
            },
            grpc_settings: if network_type == String::from("grpc") {
                Some(GRPCSettings {
                    authority: data.authority,
                    multi_mode: Some(false),
                    service_name: data.service_name,
                })
            } else {
                None
            },
            quic_settings: if network_type == String::from("quic") {
                Some(QuicSettings {
                    header: Some(NonHeaderObject {
                        r#type: Some(String::from("none")),
                    }),
                    security: Some(String::from("none")),
                    key: Some(String::from("")),
                })
            } else {
                None
            },
            kcp_settings: if network_type == String::from("kcp") {
                Some(KCPSettings {
                    mtu: None,
                    tti: None,
                    congestion: None,
                    uplink_capacity: None,
                    read_buffer_size: None,
                    write_buffer_size: None,
                    downlink_capacity: None,
                    seed: data.seed,
                })
            } else {
                None
            },
            xhttp_settings: if network_type == String::from("xhttp") {
                Some(XHTTPSettings {
                    host: data.host.clone(),
                    path: data.path.clone(),
                    mode: data.mode,
                    extra: data.extra.and_then(|e| parse_raw_json(e.as_str())),
                })
            } else {
                None
            },
        },
        settings: outbound_settings,
    };

    return outbound;
}

fn get_uri_data(uri: &str) -> (String, RawData, OutboundSettings) {
    let protocol = uri_identifier::get_uri_protocol(uri);
    return match protocol {
        Some(uri_identifier::Protocols::Vless) => {
            let d = vless::data::get_data(uri);
            let s = vless::create_outbound_settings(&d);
            (String::from("vless"), d, s)
        }
        Some(uri_identifier::Protocols::Vmess) => {
            let d = vmess::data::get_data(uri);
            let s = vmess::create_outbound_settings(&d);
            (String::from("vmess"), d, s)
        }
        Some(uri_identifier::Protocols::Trojan) => {
            let d = trojan::data::get_data(uri);
            let s = trojan::create_outbound_settings(&d);
            (String::from("trojan"), d, s)
        }
        Some(uri_identifier::Protocols::Shadowsocks) => {
            let d = shadow_socks::data::get_data(uri);
            let s = shadow_socks::create_outbound_settings(&d);
            (String::from("shadowsocks"), d, s)
        }
        Some(uri_identifier::Protocols::Socks) => {
            let d = socks::data::get_data(uri);
            let s = socks::create_outbound_settings(&d);
            (String::from("socks"), d, s)
        }
        Some(_) => {
            panic!("The protocol was recognized but is not supported yet");
        }
        None => {
            panic!("The protocol is not supported");
        }
    };
}
