use crate::services::config::Config;
use clap::Args;
use crate::entities::xray_server::XRayServer;

#[derive(Clone, Debug, Args)]
pub(crate) struct RemoveParams;

impl RemoveParams {
    pub fn remove(&self) -> anyhow::Result<()> {
        let mut repo: Config = Config::read_or_default();
        let items: Vec<XRayServer> = repo
            .list().into_iter().map(ToOwned::to_owned).collect();
        let indexes = dialoguer::MultiSelect::new().items(items).interact();

        let Ok(indexes) = indexes else {
            eprintln!("No URLs selected");
            return Ok(());
        };

        repo.remove(indexes)?;

        Ok(())
    }
}
