use crate::entities::xray_server::XRayServer;
use crate::services::config::Config;
use clap::Args;

#[derive(Clone, Debug, Args)]
pub(crate) struct RemoveParams;

impl RemoveParams {
    pub fn remove(&self) -> anyhow::Result<()> {
        let mut repo: Config = Config::read_or_default();
        let items: Vec<XRayServer> = repo.list().into_iter().map(ToOwned::to_owned).collect();
        let indexes = dialoguer::MultiSelect::new()
            .with_prompt("Select XRay server URLs to remove")
            .items(items)
            .interact();

        let Ok(indexes) = indexes else {
            eprintln!("No URLs selected");
            return Ok(());
        };

        repo.remove_by_indexes(indexes)?;

        Ok(())
    }
}
