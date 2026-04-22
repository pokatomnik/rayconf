use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use crate::entities::remote_decoder::RemoteDecoder;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Remote {
    title: String,
    url: String,
    decoder: RemoteDecoder,
}

impl Remote {
    pub fn new(title: impl AsRef<str>, url: impl AsRef<str>, decoder: RemoteDecoder) -> Self {
        Self { title: title.as_ref().to_owned(), url: url.as_ref().to_owned(), decoder }
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
}

impl Display for Remote {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.title)
    }
}