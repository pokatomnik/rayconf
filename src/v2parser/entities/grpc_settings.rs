use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct GRPCSettings {
    #[serde(rename = "authority")]
    pub authority: Option<String>,
    
    #[serde(rename = "multiMode")]
    pub multi_mode: Option<bool>,
    
    #[serde(rename = "serviceName")]
    pub service_name: Option<String>,
}