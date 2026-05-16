#[cfg(test)]
mod tests {
    use super::{parse_trojan_address, TrojanAddress};

    #[test]
    fn parses_simple_address() {
        let result = parse_trojan_address("uuid@127.0.0.1:8080");
        assert_eq!(result.uuid, "uuid");
        assert_eq!(result.address, "127.0.0.1");
        assert_eq!(result.port, 8080);
    }

    #[test]
    fn parses_percent_encoded_uuid() {
        let result = parse_trojan_address("u%20i%20d@1.2.3.4:1234");
        assert_eq!(result.uuid, "u i d");
        assert_eq!(result.address, "1.2.3.4");
        assert_eq!(result.port, 1234);
    }

    #[test]
    fn strips_trailing_slash() {
        let result = parse_trojan_address("uuid@127.0.0.1:8080/");
        assert_eq!(result.uuid, "uuid");
        assert_eq!(result.address, "127.0.0.1");
        assert_eq!(result.port, 8080);
    }

    #[test]
    fn handles_domain_name() {
        let result = parse_trojan_address("uuid@sub.example.com:8443");
        assert_eq!(result.uuid, "uuid");
        assert_eq!(result.address, "sub.example.com");
        assert_eq!(result.port, 8443);
    }
}
