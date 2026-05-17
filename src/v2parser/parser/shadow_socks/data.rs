use http::Uri;

use crate::v2parser::entities::raw_data::RawData;
use crate::v2parser::parser::shadow_socks::models::ShadowSocksAddress;
use crate::v2parser::utils::incorrect_uri::IncorrectURI;
use crate::v2parser::utils::{url_decode, url_decode_str};
use base64::{Engine, engine::general_purpose};

pub fn get_data(uri: &str) -> anyhow::Result<RawData> {
    let data = uri.split_once("ss://").incorrect_uri()?.1;
    let (raw_data, name) = data.split_once("#").unwrap_or((data, ""));
    let (raw_uri, _) = raw_data.split_once("?").unwrap_or((raw_data, ""));
    let parsed_address = parse_ss_address(raw_uri)?;
    let raw_data = RawData {
        remarks: url_decode(Some(String::from(name))).unwrap_or(String::from("")),
        server_method: url_decode(Some(parsed_address.method)),
        address: Some(parsed_address.address),
        port: Some(parsed_address.port),
        uuid: url_decode(Some(parsed_address.password)),
        r#type: Some(String::from("tcp")),
        header_type: Some(String::from("none")),
        security: None,
        fp: None,
        sni: None,
        pbk: None,
        sid: None,
        key: None,
        spx: None,
        flow: None,
        path: None,
        host: None,
        seed: None,
        mode: None,
        slpn: None,
        alpn: None,
        extra: None,
        authority: None,
        encryption: None,
        service_name: None,
        quic_security: None,
        allow_insecure: None,
        vnext_security: None,
        username: None,
    };

    Ok(raw_data)
}

fn parse_ss_address(raw_data: &str) -> anyhow::Result<ShadowSocksAddress> {
    let (userinfo, raw_address): (String, &str) = match raw_data.split_once("@") {
        None => {
            anyhow::bail!("Wrong shadowsocks format, no `@` found in the address");
        }
        Some(data) => (String::from(data.0), data.1),
    };
    let address_wo_slash = raw_address.strip_suffix("/").unwrap_or(raw_address);

    let parsed = address_wo_slash.parse::<Uri>().incorrect_uri()?;

    let method_and_password = general_purpose::STANDARD
        .decode(url_decode_str(&userinfo).unwrap_or(userinfo))
        .expect("User info is not base64");

    let (method, password) = std::str::from_utf8(&method_and_password)
        .expect("Base64 did not yield a valid utf-8 string")
        .split_once(":")
        .expect("No `:` found in the decoded base64");

    return Ok(ShadowSocksAddress {
        method: String::from(method),
        password: String::from(password),
        address: parsed.host().incorrect_uri()?.to_string(),
        port: parsed.port().incorrect_uri()?.as_u16(),
    });
}

// Tests for the internal `parse_ss_address` function
#[cfg(test)]
mod tests {
    use super::*;
    use base64::{Engine, engine::general_purpose};
    use urlencoding;

    #[test]
    fn errors_when_missing_at_symbol() {
        let raw = "example.com:8388";
        let result = parse_ss_address(&raw);
        assert!(result.is_err());
    }

    fn encode_userinfo(method: &str, password: &str) -> String {
        let combined = format!("{}:{}", method, password);
        general_purpose::STANDARD.encode(combined.as_bytes())
    }

    #[test]
    fn parses_basic_address_without_trailing_slash() {
        let method = "aes-256-cfb";
        let password = "myPass";
        let userinfo = encode_userinfo(method, password);
        let raw = format!("{}@example.com:8388", userinfo);
        let result = parse_ss_address(&raw).unwrap();
        assert_eq!(result.method, method);
        assert_eq!(result.password, password);
        assert_eq!(result.address, "example.com");
        assert_eq!(result.port, 8388);
    }

    #[test]
    fn parses_address_with_trailing_slash() {
        let method = "aes-256-cfb";
        let password = "myPass";
        let userinfo = encode_userinfo(method, password);
        let raw = format!("{}@example.com:8388/", userinfo);
        let result = parse_ss_address(&raw).unwrap();
        assert_eq!(result.address, "example.com");
        assert_eq!(result.port, 8388);
    }

    #[test]
    fn parses_percent_encoded_userinfo() {
        let method = "aes-256-cfb";
        let password = "myPass";
        let userinfo = encode_userinfo(method, password);
        let encoded = urlencoding::encode(&userinfo).into_owned();
        let raw = format!("{}@example.com:8388", encoded);
        let result = parse_ss_address(&raw).unwrap();
        assert_eq!(result.method, method);
        assert_eq!(result.password, password);
    }
}
