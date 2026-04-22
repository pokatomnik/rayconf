use crate::entities::remote::Remote;
use crate::entities::xray_server::XRayServer;
use crate::services::config::Config;
use crate::utils::tap::Tap;
use clap::Args;
use reqwest::blocking::Client;

#[derive(Debug, Args)]
pub(crate) struct SelectParams {
    #[arg(long, short, default_value_t = false, help = "Should use subscription servers")]
    remote: bool,
}

impl SelectParams {
    fn select_local(&self) -> anyhow::Result<()> {
        let config = Config::read_or_default();
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
            eprintln!("No server url selected");
            return Ok(());
        };
        println!("{}", &item.url().to_string());

        Ok(())
    }

    fn select_remote(&self) -> anyhow::Result<()> {
        let config = Config::read_or_default();
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
            .send()?
            .text()?;
        let xray_servers = selected_remote
            .decoder()
            .decode(content)
            .iter()
            .filter_map(|u| XRayServer::try_from(u.to_string()).ok())
            .collect::<Vec<XRayServer>>();

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

        println!("{}", &selected_xray_server.url().to_string());

        Ok(())
    }

    pub fn select(&self) -> anyhow::Result<()> {
        match self.remote {
            true => self.select_remote(),
            false => self.select_local(),
        }
    }
}
