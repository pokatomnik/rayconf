use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct SniffingSettings {
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,

    #[serde(rename = "destOverride")]
    pub dest_override: Option<Vec<String>>,

    #[serde(rename = "domainsExcluded")]
    pub domains_excluded: Option<Vec<String>>,

    #[serde(rename = "metadataOnly")]
    pub metadata_only: Option<bool>,

    #[serde(rename = "routeOnly")]
    pub route_only: Option<bool>,
}
