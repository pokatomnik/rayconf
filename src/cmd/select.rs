use crate::services::config::Config;
use clap::Args;
use crate::entities::xray_server::XRayServer;

#[derive(Debug, Args)]
pub(crate) struct SelectParams;

impl SelectParams {
    pub fn select(&self) -> anyhow::Result<()> {
        let config = Config::read_or_default();
        let items: Vec<XRayServer> = config
            .list()
            .into_iter()
            .map(ToOwned::to_owned)
            .collect();
        let idx = dialoguer::Select::new().items(items).interact()?;
        let Some(item) = config.server_urls.get(idx) else {
            eprintln!("No server url selected");
            return Ok(());
        };
        println!("{}", &item.url().to_string());

        Ok(())
    }
}
