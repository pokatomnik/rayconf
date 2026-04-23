use serde::{Deserialize, Serialize};
use crate::v2parser::entities::vnext_server_object::VNextServerObject;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct VLessOutboundSettings {
    #[serde(rename = "vnext")]
    pub vnext: Vec<VNextServerObject>,
}
