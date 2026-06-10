use std::cmp::Ordering;
use std::collections::HashMap;
use std::time::Duration;

use crate::entities::remote::Remote;
use crate::entities::xray_server::{XRayServer, XRayServerWithDuration};
use crate::services::config::Config;
use crate::utils::tap::Tap;
use crate::v2parser::entities::log::{Log, LogLevel};
use crate::v2parser::parser::create_json_config;
use clap::Args;
use futures::{StreamExt, stream};
use reqwest::Client;

static DEFAULT_HTTP_PORT: u16 = 8080;
static DEFAULT_SOCKS_PORT: u16 = 1080;

#[derive(Debug, Args)]
#[clap(rename_all = "kebab-case")]
pub(crate) struct SelectParams {
    #[arg(
        long,
        short,
        default_value_t = false,
        help = "Should use subscription servers"
    )]
    remote: bool,

    #[arg(long, conflicts_with = "http_port", help = format!("SOCKS5 port, default: {DEFAULT_SOCKS_PORT}"))]
    socks_port: Option<u16>,

    #[arg(long, conflicts_with = "socks_port", help = format!("HTTP port, default: {DEFAULT_HTTP_PORT}"))]
    http_port: Option<u16>,

    #[arg(long, short, help = "Log level")]
    log_level: Option<LogLevel>,

    #[arg(long, default_value_t = false, help = "Log DNS queries")]
    log_dns: bool,
}

impl SelectParams {
    async fn sort_servers<'a>(&self, servers: &'a [XRayServer]) -> Vec<XRayServerWithDuration> {
        let mut sorted_map = HashMap::new();
        let measure_results = stream::iter(servers.iter().enumerate())
            .map(async move |(idx, s)| (idx, s.measure_rtt(Duration::from_secs(5)).await))
            .buffer_unordered(5)
            .collect::<Vec<_>>()
            .await;
        for (idx, duration) in measure_results.into_iter() {
            let Some(server) = servers.get(idx) else {
                continue;
            };
            sorted_map.insert(server.to_owned(), duration);
        }

        let mut servers_sorted = sorted_map
            .into_iter()
            .collect::<Vec<(XRayServer, Option<Duration>)>>();
        servers_sorted.sort_by(
            |(_, duration_a), (_, duration_b)| match (duration_a, duration_b) {
                (None, None) => Ordering::Equal,
                (None, Some(_)) => Ordering::Greater,
                (Some(_), None) => Ordering::Less,
                (Some(duration_a), Some(duration_b)) => {
                    duration_a.as_millis().cmp(&duration_b.as_millis())
                }
            },
        );

        servers_sorted
            .into_iter()
            .map(|(server, duration)| XRayServerWithDuration::new(server, duration))
            .collect()
    }

    async fn select_local(&self) -> anyhow::Result<String> {
        let config = Config::read_or_default().await;
        let items: Vec<XRayServer> = config
            .server_urls()
            .into_iter()
            .map(ToOwned::to_owned)
            .collect();

        if items.is_empty() {
            return Err(anyhow::Error::msg("Add at least one server item"));
        }

        let idx = dialoguer::FuzzySelect::new()
            .with_prompt("Select one XRay server")
            .items(&items)
            .tap(|d| match &items.is_empty() {
                true => d,
                false => d.default(0),
            })
            .default(0)
            .interact()?;
        let server_urls = config.server_urls();
        let Some(item) = server_urls.get(idx) else {
            return Err(anyhow::Error::msg("No server url selected"));
        };

        Ok(item.url().to_string())
    }

    async fn select_remote(&self) -> anyhow::Result<String> {
        let config = Config::read_or_default().await;
        let remotes: Vec<Remote> = config
            .remotes()
            .into_iter()
            .map(ToOwned::to_owned)
            .collect();

        if remotes.is_empty() {
            return Err(anyhow::Error::msg("Add at least one remote URL"));
        }

        let remote_idx = dialoguer::FuzzySelect::new()
            .with_prompt("Select remote subscription URL")
            .items(&remotes)
            .tap(|s| match &remotes.is_empty() {
                true => s,
                false => s.default(0),
            })
            .interact()?;

        let selected_remote = remotes
            .get(remote_idx)
            .ok_or_else(|| anyhow::Error::msg("No remote URL"))?;

        let content = Client::builder()
            .build()?
            .get(selected_remote.url())
            .send()
            .await?
            .text()
            .await?;

        let xray_servers = selected_remote
            .decoder()
            .decode(content)
            .iter()
            .filter_map(|u| XRayServer::try_from(u.to_string()).ok())
            .collect::<Vec<XRayServer>>();

        let sorted = self.sort_servers(xray_servers.as_slice()).await;

        if xray_servers.is_empty() {
            return Err(anyhow::Error::msg("This URL has note XRay servers"));
        }

        let server_idx = dialoguer::FuzzySelect::new()
            .with_prompt("Select one XRay server")
            .items(&sorted)
            .tap(|d| match &xray_servers.is_empty() {
                true => d,
                false => d.default(0),
            })
            .interact()?;

        let selected_xray_server = xray_servers
            .get(server_idx)
            .ok_or_else(|| anyhow::Error::msg("No XRay server selected"))?;

        Ok(selected_xray_server.url().to_string())
    }

    fn get_log(&self) -> Log {
        let mut log = Log::default();
        log.with_log_level(self.log_level.unwrap_or_default());
        log.with_dns_log(self.log_dns);
        log
    }

    pub async fn select(&self) -> anyhow::Result<()> {
        let result = match self.remote {
            true => self.select_remote().await,
            false => self.select_local().await,
        };

        let Ok(url) = result else {
            eprintln!("No URL selected");
            return Ok(());
        };

        let socks_port = match (self.socks_port, self.http_port) {
            (None, None) => Some(DEFAULT_SOCKS_PORT),
            (Some(socks_port), None) | (Some(socks_port), Some(_)) => Some(socks_port),
            (None, Some(_)) => None,
        };

        let http_port = match (self.socks_port, self.http_port) {
            (Some(_), None) | (Some(_), Some(_)) | (None, None) => None,
            (None, Some(http_port)) => Some(http_port),
        };

        let config_json =
            create_json_config(url.as_str(), socks_port, http_port, Some(self.get_log()))?;

        println!("{}", config_json);

        Ok(())
    }
}
