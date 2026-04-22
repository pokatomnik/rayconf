use base64::prelude::*;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(ValueEnum, Clone, Copy, Debug, Serialize, Deserialize)]
#[clap(rename_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub(crate) enum RemoteDecoder {
    /// If remote URL has plaintext URLs
    Plain,

    /// If remote URL has Base64-encoded URLs
    Base64,
}

impl AsRef<RemoteDecoder> for RemoteDecoder {
    fn as_ref(&self) -> &RemoteDecoder {
        &self
    }
}

impl RemoteDecoder {
    fn decode_plain(&self, source: impl AsRef<str>) -> Vec<String> {
        let source = source.as_ref();
        match source.is_empty() {
            true => vec![],
            false => source.split('\n').map(String::from).collect(),
        }
    }

    fn decode_base64(&self, source: impl AsRef<str>) -> Vec<String> {
        let source = source.as_ref();
        let decoded_vec = BASE64_STANDARD
            .decode(source.as_bytes())
            .unwrap_or_default();
        let str = String::from_utf8_lossy(&decoded_vec).to_string();

        self.decode_plain(&str)
    }

    pub fn decode(&self, source: impl AsRef<str>) -> Vec<String> {
        match &self {
            RemoteDecoder::Plain => self.decode_plain(source),
            RemoteDecoder::Base64 => self.decode_base64(source),
        }
    }
}

impl Display for RemoteDecoder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            RemoteDecoder::Plain => f.write_str("plain"),
            RemoteDecoder::Base64 => f.write_str("base64"),
        }
    }
}
