use percent_encoding::percent_decode_str;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use url::Url;

const UNNAMED: &'static str = "Unnamed XRay Server";

#[derive(Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq, Eq, Ord, Hash)]
#[serde(transparent)]
pub(crate) struct XRayServer(Url);

impl XRayServer {
    pub fn url(&self) -> &Url {
        &self.0
    }
}

impl Display for XRayServer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = self
            .0
            .fragment()
            .map(percent_decode_str)
            .and_then(|v| v.decode_utf8().to_owned().ok())
            .map(|v| v.to_string())
            .unwrap_or_else(|| UNNAMED.to_string());
        write!(f, "{}", str)
    }
}

impl TryFrom<String> for XRayServer {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let url = Url::parse(value.as_str())?;
        let url = XRayServer(url);
        Ok(url)
    }
}
