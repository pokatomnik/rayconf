use crate::entities::remote_decoder::RemoteDecoder;
use crate::services::config::Config;
use clap::{Args, ValueEnum};
use url::Url;

#[derive(ValueEnum, Clone, Copy, Debug)]
#[clap(rename_all = "kebab-case")]
pub(crate) enum RemoteAction {
    Add,
    Remove,
    List,
}

#[derive(Clone, Debug, Args)]
#[clap(rename_all = "kebab-case")]
pub(crate) struct RemoteParams {
    action: RemoteAction,
}

impl RemoteParams {
    async fn handle_add(&self) -> anyhow::Result<()> {
        let mut config = Config::read_or_default().await;

        let title: String = dialoguer::Input::new()
            .with_prompt("Specify remote alias")
            .interact_text()?;
        let url = dialoguer::Input::new()
            .with_prompt("Specify remote URL")
            .validate_with(|v: &String| match Url::parse(v) {
                Ok(_) => Ok(()),
                Err(_) => Err(anyhow::Error::msg("Incorrect remote URL")),
            })
            .interact_text()?;
        let decoders = vec![RemoteDecoder::Plain, RemoteDecoder::Base64];
        let decoder = dialoguer::FuzzySelect::new()
            .with_prompt("Specify remote decoder")
            .items(&decoders)
            .default(0)
            .interact()?;
        let Some(decoder) = decoders.get(decoder) else {
            return Err(anyhow::Error::msg("Remote decoder does not exist"));
        };

        config.add_remote(title, url, decoder).await?;

        Ok(())
    }

    async fn handle_remove(&self) -> anyhow::Result<()> {
        let mut config = Config::read_or_default().await;
        let remove_idx = dialoguer::FuzzySelect::new()
            .with_prompt("Select remote")
            .items(config.remotes())
            .interact()?;
        config.remove_remote_by_indexes(vec![remove_idx]).await
    }

    async fn handle_list(&self) -> anyhow::Result<()> {
        let config = Config::read_or_default().await;
        let remotes = config.remotes();
        if remotes.is_empty() {
            return Err(anyhow::Error::msg("No remote found"));
        }
        for remote in remotes {
            println!("{}: {}", remote.title(), remote.url());
        }

        Ok(())
    }

    pub async fn handle_action(&self) -> anyhow::Result<()> {
        match &self.action {
            RemoteAction::Add => self.handle_add().await,
            RemoteAction::Remove => self.handle_remove().await,
            RemoteAction::List => self.handle_list().await,
        }
    }
}
