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

    async fn measure_rtt_for_addr(
        &self,
        timeout: Duration,
        addr: &SocketAddr,
    ) -> anyhow::Result<Duration> {
        let start = Instant::now();
        match tokio::time::timeout(timeout, tokio::net::TcpStream::connect(addr)).await {
            Ok(Ok(stream)) => {
                let result = start.elapsed();
                drop(stream);
                Ok(result)
            }
            Ok(Err(err)) => Err(err.into()),
            _ => anyhow::bail!("Timeout"),
        }
    }

    async fn measure_rtt_for_addrs(
        &self,
        timeout: Duration,
        addrs: impl Iterator<Item = SocketAddr>,
    ) -> anyhow::Result<Duration> {
        let mut durations = Vec::new();
        for addr in addrs {
            let duration = self.measure_rtt_for_addr(timeout, &addr).await;
            durations.push(duration);
        }
        if durations.is_empty() {
            anyhow::bail!("Empty addrs list");
        }
        let mut ok_durations = durations
            .into_iter()
            .filter_map(|r| r.ok())
            .collect::<Vec<Duration>>();
        ok_durations.sort_by_key(|v| v.as_millis());

        ok_durations
            .get((ok_durations.len() / 2) as usize)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Cannot estimate"))
    }

    pub async fn measure_rtt(&self, timeout: Duration) -> Option<Duration> {
        let Some((host, port)) = self.url().host().zip(self.0.port()) else {
            return None;
        };
        let ip_addresses = match host {
            Host::Domain(dns_name) => dns_client::resolve(dns_name).await,
            Host::Ipv4(ipv4) => Arc::new(vec![IpAddr::V4(ipv4)]),
            Host::Ipv6(ipv6) => Arc::new(vec![IpAddr::V6(ipv6)]),
        };
        let socket_addrs = ip_addresses
            .iter()
            .map(|v| SocketAddr::new(v.to_owned(), port));

        self.measure_rtt_for_addrs(timeout, socket_addrs).await.ok()
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
