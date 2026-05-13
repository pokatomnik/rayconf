use crate::services::dns_client;
use percent_encoding::percent_decode_str;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::Instant;
use url::{Host, Url};

const UNNAMED: &'static str = "Unnamed XRay Server";

#[derive(Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq, Eq, Ord, Hash)]
#[serde(transparent)]
pub(crate) struct XRayServer(Url);

impl XRayServer {
    pub fn url(&self) -> &Url {
        &self.0
    }

    pub async fn measure_rtt(&self, timeout: Duration) -> anyhow::Result<Option<Duration>> {
        let Some((host, port)) = self.url().host().zip(self.0.port()) else {
            return Ok(None);
        };
        let ip_addresses = match host {
            Host::Domain(dns_name) => dns_client::resolve(dns_name).await,
            Host::Ipv4(ipv4) => Arc::new(vec![IpAddr::V4(ipv4)]),
            Host::Ipv6(ipv6) => Arc::new(vec![IpAddr::V6(ipv6)]),
        };

        let Some(ip_addr) = ip_addresses.first().cloned() else {
            return Ok(None);
        };

        let start = Instant::now();

        let addr = SocketAddr::new(ip_addr, port);
        let conn_future = tokio::net::TcpStream::connect(addr);
        let result = match tokio::time::timeout(timeout, conn_future).await {
            Ok(Ok(stream)) => {
                let result = Some(start.elapsed().as_millis());
                drop(stream);
                Some(result)
            }
            _ => anyhow::bail!("Timeout"),
        }
        .flatten();

        Ok(result.map(|v| Duration::from_millis(v as u64)))
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
