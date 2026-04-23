use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct SocksUser {
    #[serde(rename = "user")]
    pub user: Option<String>,

    #[serde(rename = "pass")]
    pub pass: Option<String>,
}
