use crate::entities::remote_decoder::RemoteDecoder;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Remote {
    #[serde(rename = "title")]
    title: String,

    #[serde(rename = "url")]
    url: String,

    #[serde(rename = "decoder")]
    decoder: RemoteDecoder,

    #[serde(rename = "headers")]
    headers: Option<HashMap<String, String>>,
}

impl Remote {
    pub fn new(
        title: impl AsRef<str>,
        url: impl AsRef<str>,
        decoder: RemoteDecoder,
        optional_headers: Option<HashMap<String, String>>,
    ) -> Self {
        Self {
            title: title.as_ref().to_owned(),
            url: url.as_ref().to_owned(),
            decoder,
            headers: optional_headers,
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn decoder(&self) -> RemoteDecoder {
        self.decoder
    }

    pub fn headers(&self) -> Option<&HashMap<String, String>> {
        self.headers.as_ref()
    }
}

impl Display for Remote {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.title)
    }
}
