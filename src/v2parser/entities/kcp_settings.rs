use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct KCPSettings {
    #[serde(rename = "mtu")]
    pub mtu: Option<u32>,

    #[serde(rename = "tti")]
    pub tti: Option<u32>,

    #[serde(rename = "uplinkCapacity")]
    pub uplink_capacity: Option<u32>,

    #[serde(rename = "downlinkCapacity")]
    pub downlink_capacity: Option<u32>,

    #[serde(rename = "congestion")]
    pub congestion: Option<bool>,

    #[serde(rename = "readBufferSize")]
    pub read_buffer_size: Option<u32>,

    #[serde(rename = "writeBufferSize")]
    pub write_buffer_size: Option<u32>,

    #[serde(rename = "seed")]
    pub seed: Option<String>,
}
