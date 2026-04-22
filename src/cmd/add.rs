use crate::services::config::Config;
use clap::Args;
use url::Url;

#[derive(Debug, Args)]
pub(crate) struct AddParams {
    /// XRay server URL
    url: Option<Url>,
}

impl AddParams {
    pub fn add(&self) -> anyhow::Result<()> {
        let mut repo: Config = Config::read_or_default();
        let url = match self.url {
            Some(ref url) => Some(url.to_owned()),
            None => dialoguer::Input::new()
                .with_prompt("Specify a new XRay outbound URL")
                .interact_text()
                .ok(),
        };
        let Some(url) = url else {
            eprintln!("URL not specified or can't be parsed");
            return Ok(());
        };
        repo.add_local(url)?;
        Ok(())
    }
}
