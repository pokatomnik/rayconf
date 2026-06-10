use dnsclient::UpstreamServer;
use dnsclient::r#async::DNSClient as AsyncDNSClient;
use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::sync::{Arc, LazyLock};
use tokio::sync::{Mutex, OnceCell};

static DNS_CLIENTS_RAW: &'static str = include_str!("./dns_servers.txt");
static SEP: &'static str = "\n";

struct DNSClient {
    client: AsyncDNSClient,
    cache: Arc<Mutex<HashMap<String, Arc<OnceCell<Arc<Vec<IpAddr>>>>>>>,
}

impl DNSClient {
    fn try_new() -> anyhow::Result<Self> {
        let dns_servers: Vec<UpstreamServer> = DNS_CLIENTS_RAW
            .split(SEP)
            .filter_map(|v| -> Option<SocketAddr> { v.parse().ok() })
            .map(|v| UpstreamServer::new(v))
            .collect();

        if dns_servers.is_empty() {
            return Err(anyhow::Error::msg("No DNS servers found"));
        }

        let client = AsyncDNSClient::new(dns_servers);

        Ok(Self {
            client,
            cache: Arc::default(),
        })
    }

    async fn get_all_addrs(&self, dns_name: impl AsRef<str>) -> Vec<IpAddr> {
        let ipv4 = self
            .client
            .query_a(dns_name.as_ref())
            .await
            .unwrap_or_else(|_| vec![])
            .into_iter()
            .map(|v| IpAddr::V4(v))
            .collect::<Vec<IpAddr>>();
        let ipv6 = self
            .client
            .query_aaaa(dns_name.as_ref())
            .await
            .unwrap_or_else(|_| vec![])
            .into_iter()
            .map(|v| IpAddr::V6(v))
            .collect::<Vec<IpAddr>>();
        let mut result = Vec::with_capacity(ipv4.len() + ipv6.len());

        result.extend(ipv4);
        result.extend(ipv6);

        result
    }

    async fn resolve_or_insert(
        &self,
        dns_name: impl AsRef<str>,
    ) -> Arc<OnceCell<Arc<Vec<IpAddr>>>> {
        let mut cache = self.cache.lock().await;
        let result = cache
            .entry(dns_name.as_ref().to_owned())
            .or_insert_with(|| Arc::new(OnceCell::new()));

        result.clone()
    }

    pub async fn resolve(&self, dns_name: impl AsRef<str>) -> Arc<Vec<IpAddr>> {
        let cell = self.resolve_or_insert(dns_name.as_ref()).await;
        cell.get_or_init(async move || {
            let addrs = self.get_all_addrs(dns_name.as_ref()).await;
            Arc::new(addrs)
        })
        .await
        .clone()
    }
}

static DEFAULT_DNS_CLIENT: LazyLock<anyhow::Result<DNSClient>> = LazyLock::new(DNSClient::try_new);

pub(crate) async fn resolve(dns_name: impl AsRef<str>) -> Arc<Vec<IpAddr>> {
    match DEFAULT_DNS_CLIENT.as_ref() {
        Err(_) => Arc::default(),
        Ok(client) => client.resolve(dns_name).await,
    }
}
