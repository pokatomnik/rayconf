use crate::utils::to_err::ToAnyhow;
use crate::v2parser::entities::raw_data::RawData;
use crate::v2parser::parser::trojan::models::TrojanAddress;
use crate::v2parser::utils::{get_parameter_value, url_decode};
use http::Uri;

pub fn get_data(uri: &str) -> anyhow::Result<RawData> {
    let data = uri
        .split_once("trojan://")
        .anyhow("Incorrect URI format")?
        .1;
    let query_and_name = uri.split_once("?").anyhow("Incorrect URI format")?.1;
    let (raw_query, name) = query_and_name
        .split_once("#")
        .unwrap_or((query_and_name, ""));
    let trojan_address = data.split_once("?").anyhow("Incorrect URI format")?.0;
    let parsed_address = parse_trojan_address(trojan_address)?;
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

fn parse_trojan_address(raw_data: &str) -> anyhow::Result<TrojanAddress> {
    let (uuid, raw_address): (String, &str) = match raw_data.split_once("@") {
        None => {
            anyhow::bail!("Wrong trojan format, no `@` found in the address");
        }
        Some(data) => (String::from(data.0), data.1),
    };
    let address_wo_slash = raw_address.strip_suffix("/").unwrap_or(raw_address);

    let parsed = address_wo_slash.parse::<Uri>().unwrap();

    let result = TrojanAddress {
        uuid: url_decode(Some(uuid)).unwrap(),
        address: parsed.host().unwrap().to_string(),
        port: parsed.port().unwrap().as_u16(),
    };

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::parse_trojan_address;

    #[test]
    fn parse_trojan_address_trims_trailing_slash() {
        let result = parse_trojan_address("test-pw%2Bencoded@13.50.100.84:22222/").unwrap();

        assert_eq!(result.uuid, "test-pw+encoded");
        assert_eq!(result.address, "13.50.100.84");
        assert_eq!(result.port, 22222);
    }

    #[test]
    fn parse_trojan_address_without_trailing_slash() {
        let result = parse_trojan_address("plain-password@trj.rollingnext.co.uk:443").unwrap();

        assert_eq!(result.uuid, "plain-password");
        assert_eq!(result.address, "trj.rollingnext.co.uk");
        assert_eq!(result.port, 443);
    }

    #[test]
    fn parse_trojan_address_returns_error_when_at_is_missing() {
        let result = parse_trojan_address("plain-password13.50.100.84:443");

        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "Wrong trojan format, no `@` found in the address"
        );
    }

    #[test]
    fn parse_trojan_address_returns_error_for_empty_input() {
        let result = parse_trojan_address("");

        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "Wrong trojan format, no `@` found in the address"
        );
    }
}
