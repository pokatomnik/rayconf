use crate::v2parser::entities::raw_data::RawData;
use crate::v2parser::parser::vless::models::VLessAddress;
use crate::v2parser::utils::incorrect_uri::IncorrectURI;
use crate::v2parser::utils::{get_parameter_value, url_decode};
use http::Uri;

pub fn get_data(uri: &str) -> anyhow::Result<RawData> {
    let data = uri.split_once("vless://").incorrect_uri()?.1;
    let query_and_name = uri.split_once("?").incorrect_uri()?.1;
    let (raw_query, name) = query_and_name
        .split_once("#")
        .unwrap_or((query_and_name, ""));
    let parsed_address = parse_vless_address(data.split_once("?").incorrect_uri()?.0)?;
    let query: Vec<(&str, &str)> = querystring::querify(raw_query);

    let result = RawData {
        remarks: url_decode(Some(String::from(name))).unwrap_or(String::from("")),
        uuid: Some(parsed_address.uuid),
        port: Some(parsed_address.port),
        address: Some(parsed_address.address),
        alpn: url_decode(get_parameter_value(&query, "alpn")),
        path: url_decode(get_parameter_value(&query, "path")),
        authority: url_decode(get_parameter_value(&query, "authority")),
        pbk: url_decode(get_parameter_value(&query, "pbk")),
        security: get_parameter_value(&query, "security"),
        sid: url_decode(get_parameter_value(&query, "sid")),
        flow: get_parameter_value(&query, "flow"),
        sni: get_parameter_value(&query, "sni"),
        fp: url_decode(get_parameter_value(&query, "fp")),
        r#type: get_parameter_value(&query, "type"),
        encryption: get_parameter_value(&query, "encryption"),
        header_type: get_parameter_value(&query, "headerType"),
        host: url_decode(get_parameter_value(&query, "host")),
        seed: url_decode(get_parameter_value(&query, "seed")),
        quic_security: get_parameter_value(&query, "quicSecurity"),
        key: get_parameter_value(&query, "key"),
        mode: url_decode(get_parameter_value(&query, "mode")),
        service_name: url_decode(get_parameter_value(&query, "serviceName")),
        vnext_security: None,
        slpn: get_parameter_value(&query, "slpn"),
        spx: url_decode(get_parameter_value(&query, "spx")),
        extra: url_decode(get_parameter_value(&query, "extra")),
        allow_insecure: get_parameter_value(&query, "allowInsecure"),
        server_method: None,
        username: None,
    };

    Ok(result)
}

fn parse_vless_address(raw_data: &str) -> anyhow::Result<VLessAddress> {
    let (uuid, raw_address): (String, &str) = match raw_data.split_once("@") {
        None => {
            anyhow::bail!("Wrong vless format, no `@` found in the address");
        }
        Some(data) => (String::from(data.0), data.1),
    };
    let address_wo_slash = raw_address.strip_suffix("/").unwrap_or(raw_address);

    let parsed = address_wo_slash.parse::<Uri>().incorrect_uri()?;

    let result = VLessAddress {
        uuid: url_decode(Some(uuid)).incorrect_uri()?,
        address: parsed.host().incorrect_uri()?.to_string(),
        port: parsed.port().incorrect_uri()?.as_u16(),
    };

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::parse_vless_address;

    #[test]
    fn parse_vless_address_with_trailing_slash_and_encoded_uuid() {
        let parsed = parse_vless_address("my%2Duuid@https://example.com:443/").unwrap();

        assert_eq!(parsed.uuid, "my-uuid");
        assert_eq!(parsed.address, "example.com");
        assert_eq!(parsed.port, 443);
    }

    #[test]
    fn parse_vless_address_without_trailing_slash() {
        let parsed = parse_vless_address("uuid123@https://server.test:8443").unwrap();

        assert_eq!(parsed.uuid, "uuid123");
        assert_eq!(parsed.address, "server.test");
        assert_eq!(parsed.port, 8443);
    }
}
