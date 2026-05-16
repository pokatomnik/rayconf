use std::process::Stdio;
use std::{cmp::Ordering, collections::HashMap};

use crate::entities::remote::Remote;
use crate::entities::xray_server::XRayServer;
use crate::entities::xray_server_with_perf::XRayServerWithPerf;
use crate::services::config::Config;
use crate::services::measures::Measures;
use crate::utils::tap::Tap;
use crate::v2parser::entities::log::{Log, LogLevel};
use crate::v2parser::parser::create_json_config;
use clap::Args;
use reqwest::Client;
use tokio::io::AsyncWriteExt;

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

    #[arg(
        long,
        short,
        default_value_t = false,
        help = "Do not run XRay, just print config"
    )]
    dry_run: bool,

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
    async fn sort_servers(&self, servers: Vec<XRayServer>) -> Vec<XRayServerWithPerf> {
        let measures = Measures::read_or_default().await;

        let mut measure_results = Vec::with_capacity(servers.len());
        for server in servers.iter() {
            let measure_result = measures.get_latest_measure(server.url()).await;
            measure_results.push(measure_result);
        }

        let mut measure_results_index = HashMap::with_capacity(measure_results.len());
        for measure_result in measure_results {
            if let Some(measure_result) = measure_result {
                measure_results_index.insert(measure_result.url().to_owned(), measure_result);
            }
        }

        let mut servers = servers.clone();

        servers.sort_by(|server_a, server_b| {
            let (speed_a, speed_b) = (
                measure_results_index.get(server_a.url()),
                measure_results_index.get(server_b.url()),
            );

            match (speed_a, speed_b) {
                (None, None) => Ordering::Equal,
                (None, Some(_)) => Ordering::Greater,
                (Some(_), None) => Ordering::Less,
                (Some(a), Some(b)) => match (a.perf_data(), b.perf_data()) {
                    (None, None) => Ordering::Equal,
                    (None, Some(_)) => Ordering::Greater,
                    (Some(_), None) => Ordering::Less,
                    (Some(a), Some(b)) => b.download_speed_mbps().cmp(&a.download_speed_mbps()),
                },
            }
        });

        servers
            .into_iter()
            .map(|ref s| {
                XRayServerWithPerf::new(s.clone(), measure_results_index.get(s.url()).cloned())
            })
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

        let xray_servers = self.sort_servers(xray_servers).await;

        if xray_servers.is_empty() {
            return Err(anyhow::Error::msg("This URL has note XRay servers"));
        }

        let server_idx = dialoguer::FuzzySelect::new()
            .with_prompt("Select one XRay server")
            .items(&xray_servers)
            .tap(|d| match &xray_servers.is_empty() {
                true => d,
                false => d.default(0),
            })
            .interact()?;

        let selected_xray_server = xray_servers
            .get(server_idx)
            .ok_or_else(|| anyhow::Error::msg("No XRay server selected"))?;

        Ok(selected_xray_server.xray_server().url().to_string())
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

        match self.dry_run {
            true => {
                println!("{}", config_json);
                Ok(())
            }
            false => self.run_xray(config_json).await,
        }
    }

    async fn run_xray(&self, config: impl AsRef<str>) -> anyhow::Result<()> {
        static BIN_NAME: &'static str = "xray";
        let mut command = tokio::process::Command::new(BIN_NAME)
            .kill_on_drop(true)
            .stderr(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stdin(Stdio::piped())
            .spawn()
            .map_err(|_| anyhow::anyhow!("Failed to run XRay, please make sure It is installed"))?;

        let mut child_stdin = command
            .stdin
            .take()
            .ok_or_else(|| anyhow::anyhow!("No child stdin"))?;

        child_stdin.write_all(config.as_ref().as_bytes()).await?;

        drop(child_stdin);

        command.wait().await?;

        Ok(())
    }
}
