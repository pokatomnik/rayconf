use serde::{Deserialize, Serialize};
use crate::v2parser::entities::vnext_user::VNextUser;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct VNextServerObject {
    #[serde(rename = "address")]
    pub address: Option<String>,

    #[serde(rename = "port")]
    pub port: Option<u16>,

    #[serde(rename = "users")]
    pub users: Option<Vec<VNextUser>>,
}
