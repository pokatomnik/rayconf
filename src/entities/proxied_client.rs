use std::time::Duration;

use reqwest::{Client, ClientBuilder, Proxy};
use tokio::time::Instant;
use url::Url;

pub(crate) struct ProxiedClient(Client);

impl ProxiedClient {
    #[inline]
    fn proxy_url(proxy_host: impl AsRef<str>, proxy_port: u16) -> String {
        let proxy_host = proxy_host.as_ref();
        format!("socks5h://{proxy_host}:{proxy_port}")
    }

    #[inline]
    fn download_url(bytes_len: u128) -> anyhow::Result<Url> {
        static DOWNLOAD_URL_BASE: &'static str = "https://speed.cloudflare.com/__down";
        let mut download_url = Url::parse(DOWNLOAD_URL_BASE)?;

        let bytes_kv = format!("bytes={bytes_len}");
        download_url.set_query(Some(bytes_kv.as_str()));

        Ok(download_url)
    }

    pub fn try_new(
        proxy_host: impl AsRef<str>,
        proxy_port: u16,
        timeout: Duration,
    ) -> anyhow::Result<Self> {
        let proxy_url = Self::proxy_url(proxy_host.as_ref(), proxy_port);
        let proxy = Proxy::all(proxy_url)?;
        let client = ClientBuilder::new()
            .proxy(proxy)
            .no_gzip()
            .no_brotli()
            .no_deflate()
            .timeout(timeout)
            .build()?;

        let instance = Self(client);

        Ok(instance)
    }

    #[must_use]
    #[allow(unused)]
    pub async fn measure_first_successful(
        &self,
        bytes_len: u128,
        delay: Duration,
        attempts: usize,
    ) -> anyhow::Result<Duration> {
        for _ in 0..attempts {
            tokio::time::sleep(delay).await;
            match self.measure_attempt(bytes_len).await {
                Ok(duration) => return Ok(duration),
                _ => {}
            }
        }

        anyhow::bail!("Cannot estimate")
    }

    #[must_use]
    #[allow(unused)]
    pub async fn measure_median(
        &self,
        bytes_len: u128,
        delay: Duration,
        attempts: usize,
    ) -> anyhow::Result<Duration> {
        let mut results = Vec::with_capacity(attempts);
        for _ in 0..attempts {
            tokio::time::sleep(delay).await;
            match self.measure_attempt(bytes_len).await {
                Ok(duration) => results.push(duration),
                _ => {}
            }
        }

        if results.len() == 0 {
            anyhow::bail!("Cannot estimate, no successful measures")
        }

        results.sort_by_key(|v| v.as_millis());

        results
            .get(results.len() / 2)
            .map(|v| *v)
            .ok_or_else(|| anyhow::anyhow!("Cannot estimate"))
    }

    async fn measure_attempt(&self, bytes_len: u128) -> anyhow::Result<Duration> {
        let download_url = Self::download_url(bytes_len)?;

        let request = self
            .0
            .get(download_url)
            .header(reqwest::header::ACCEPT_ENCODING, "identity")
            .build()?;

        let start = Instant::now();
        let mut response = self.0.execute(request).await?;

        let mut bytes_read = 0u128;

        while let Some(chunk) = response.chunk().await? {
            bytes_read += chunk.len() as u128;
        }

        let elapsed = start.elapsed();

        if bytes_len != bytes_read {
            let err_msg = format!("requested bytes: {bytes_len}, but downloaded: {bytes_read}");
            let err = anyhow::anyhow!(err_msg);
            return Err(err);
        }

        Ok(elapsed)
    }
}
