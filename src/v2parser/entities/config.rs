use serde::{Deserialize, Serialize};
use crate::v2parser::entities::inbound::Inbound;
use crate::v2parser::entities::outbound::Outbound;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Config {
    pub outbounds: Vec<Outbound>,
    pub inbounds: Vec<Inbound>,
}
