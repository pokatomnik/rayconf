pub(crate) trait URLDecode {
    fn decode_as_urlencoded(&self) -> anyhow::Result<String>;
}

impl URLDecode for String {
    fn decode_as_urlencoded(&self) -> anyhow::Result<String> {
        let result = urlencoding::decode(self).map(|v| v.to_owned().to_string())?;
        Ok(result)
    }
}

impl URLDecode for &str {
    fn decode_as_urlencoded(&self) -> anyhow::Result<String> {
        let result = urlencoding::decode(self).map(|v| v.to_owned().to_string())?;
        Ok(result)
    }
}
