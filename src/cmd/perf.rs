use std::{process::Stdio, time::Duration};

use clap::Args;
use reqwest::Client;
use tokio::{io::AsyncWriteExt, process::Child, sync::OnceCell};

use crate::entities::perf_result::{PerfData, PerfResult};
use crate::entities::proxied_client::ProxiedClient;
use crate::entities::remote::Remote;
use crate::entities::xray_server::XRayServer;
use crate::services::config::Config;
use crate::services::measures::Measures;
use crate::services::portman::Portman;
use crate::utils::tap::Tap;
use crate::utils::urldecode::URLDecode;
use crate::v2parser::parser::create_json_config;

static FIFTY_MB_IN_BYTES: u128 = 50 * 1024 * 1024;
static UNKNOWN_SERVER_NAME: &'static str = "Unknown";
static PROXY_HOST: &'static str = "127.0.0.1";
static PROXY_TEST_TIMEOUT: Duration = Duration::from_mins(5);
static DELAY_BETWEEN_ATTEMPS: Duration = Duration::from_millis(200);
static PROXY_TEST_ATTEMPTS: usize = 5;

#[derive(Args)]
pub(crate) struct PerfParams {
    #[clap(skip)]
    portman: OnceCell<Portman>,

    #[clap(skip)]
    measures: OnceCell<Measures>,
}

impl PerfParams {
    async fn select_remote(&self) -> Vec<XRayServer> {
        let config = Config::read_or_default().await;
        let remotes: Vec<Remote> = config
            .remotes()
            .into_iter()
            .map(ToOwned::to_owned)
            .collect();

        if remotes.is_empty() {
            return vec![];
        }

        let remote_idx_result = dialoguer::FuzzySelect::new()
            .with_prompt("Select remote subscription URL")
            .items(&remotes)
            .tap(|s| match &remotes.is_empty() {
                true => s,
                false => s.default(0),
            })
            .interact();

        let Ok(remote_idx) = remote_idx_result else {
            return vec![];
        };

        let selected_remote = remotes
            .get(remote_idx)
            .ok_or_else(|| anyhow::Error::msg("No remote URL"));

        let Ok(selected_remote) = selected_remote else {
            return vec![];
        };

        let Ok(client) = Client::builder().build() else {
            return vec![];
        };
        let Ok(response) = client.get(selected_remote.url()).send().await else {
            return vec![];
        };
        let Ok(content) = response.text().await else {
            return vec![];
        };

        selected_remote
            .decoder()
            .decode(content)
            .iter()
            .filter_map(|u| XRayServer::try_from(u.to_string()).ok())
            .collect::<Vec<XRayServer>>()
    }

    async fn get_measures(&self) -> &Measures {
        self.measures
            .get_or_init(async || Measures::read_or_default().await)
            .await
    }

    async fn get_portman(&self) -> &Portman {
        self.portman.get_or_init(async || Portman::new()).await
    }

    async fn measure_server_speed(&self, server: &XRayServer) -> anyhow::Result<Duration> {
        server
            .measure_rtt(Duration::from_secs(5))
            .await
            .ok_or_else(|| anyhow::anyhow!("Cannot connect to a server"))?;
        let portman = self.get_portman().await;
        let url = server.url();

        let result = portman
            .lease_port(async move |port| -> anyhow::Result<Duration> {
                let config_json = create_json_config(url.as_str(), Some(port), None, None);
                let mut command = self.run_xray(config_json).await?;
                let measure_result = ProxiedClient::try_new(PROXY_HOST, port, PROXY_TEST_TIMEOUT)?
                    .measure_first_successful(
                        FIFTY_MB_IN_BYTES,
                        DELAY_BETWEEN_ATTEMPS,
                        PROXY_TEST_ATTEMPTS,
                    )
                    .await;
                command.kill().await?;
                measure_result
            })
            .await;

        if let Ok(result) = result {
            let perf_data = PerfData::from_raw_data(FIFTY_MB_IN_BYTES as u64, result);
            let perf_result = PerfResult::now(&url, Some(perf_data));
            self.get_measures()
                .await
                .add_measure(&url, perf_result)
                .await;
        }

        result
    }

    pub async fn start(&self) -> anyhow::Result<()> {
        let servers = self.select_remote().await;

        for server in servers.iter() {
            let result = self.measure_server_speed(server).await;
            let server_display_name = server
                .url()
                .fragment()
                .unwrap_or_else(|| UNKNOWN_SERVER_NAME)
                .decode_as_urlencoded()
                .unwrap_or_else(|_| UNKNOWN_SERVER_NAME.to_string());
            let output = match result {
                Ok(result) => format!(
                    "Server {server_display_name} responded with time: {} ms",
                    result.as_millis()
                ),
                Err(error) => format!(
                    "Server {server_display_name} did not responded, error: {}",
                    error.to_string()
                ),
            };
            println!("{output}");
        }

        self.get_measures().await.dump().await?;

        Ok(())
    }

    async fn run_xray(&self, config: impl AsRef<str>) -> anyhow::Result<Child> {
        static BIN_NAME: &'static str = "xray";
        let mut command = tokio::process::Command::new(BIN_NAME)
            .kill_on_drop(true)
            .stderr(Stdio::null())
            .stdout(Stdio::null())
            .stdin(Stdio::piped())
            .spawn()
            .map_err(|_| anyhow::anyhow!("Failed to run XRay, please make sure It is installed"))?;

        let mut child_stdin = command
            .stdin
            .take()
            .ok_or_else(|| anyhow::anyhow!("No child stdin"))?;

        child_stdin.write_all(config.as_ref().as_bytes()).await?;

        drop(child_stdin);

        Ok(command)
    }
}
