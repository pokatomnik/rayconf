use crate::entities::xray_server::XRayServer;
use crate::services::config::Config;
use clap::Args;
use crate::utils::tap::Tap;

#[derive(Debug, Args)]
pub(crate) struct SelectParams;

impl SelectParams {
    pub fn select(&self) -> anyhow::Result<()> {
        let config = Config::read_or_default();
        let items: Vec<XRayServer> = config.list().into_iter().map(ToOwned::to_owned).collect();
        let idx = dialoguer::Select::new()
            .with_prompt("Select one XRay server")
            .items(&items)
            .tap(|d| match &items.is_empty() {
                true => d,
                false => d.default(0),
            })
            .default(0)
            .interact()?;
        let Some(item) = config.server_urls.get(idx) else {
            eprintln!("No server url selected");
            return Ok(());
        };
        println!("{}", &item.url().to_string());

        Ok(())
    }
}
