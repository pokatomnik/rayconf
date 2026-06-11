use crate::entities::remote::Remote;
use crate::entities::remote_decoder::RemoteDecoder;
use crate::entities::xray_server::XRayServer;
use crate::services::fileman::FileMan;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

static DEFAULT_CONFIG_FILE_NAME: &'static str = "rayconf.json";

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct Config {
    #[serde(rename = "serverURLs")]
    server_urls: Vec<XRayServer>,

    #[serde(rename = "remotes")]
    remotes: Vec<Remote>,
}

impl Config {
    pub async fn add_local(&mut self, url: impl AsRef<str>) -> anyhow::Result<()> {
        let url = XRayServer::try_from(url.as_ref().to_owned())?;
        self.server_urls.push(url);

        self.try_write().await
    }

    pub async fn add_remote(
        &mut self,
        name: impl AsRef<str>,
        url: impl AsRef<str>,
        decoder: impl AsRef<RemoteDecoder>,
        headers: Option<HashMap<String, String>>,
    ) -> anyhow::Result<()> {
        let remote = Remote::new(
            name.as_ref(),
            url.as_ref(),
            decoder.as_ref().clone(),
            headers,
        );

        self.remotes.push(remote);

        self.try_write().await
    }

    pub async fn remove_locals_by_indexes(&mut self, indexes: Vec<usize>) -> anyhow::Result<()> {
        let hs = HashSet::<usize>::from_iter(indexes);
        self.server_urls = self
            .server_urls
            .iter()
            .enumerate()
            .filter_map(|(idx, item)| match hs.contains(&idx) {
                true => None,
                false => Some(item.clone()),
            })
            .collect();

        self.try_write().await
    }

    pub async fn remove_remote_by_indexes(&mut self, indexes: Vec<usize>) -> anyhow::Result<()> {
        let hs = HashSet::<usize>::from_iter(indexes);
        self.remotes = self
            .remotes
            .iter()
            .enumerate()
            .filter_map(|(idx, item)| match hs.contains(&idx) {
                true => None,
                false => Some(item.clone()),
            })
            .collect();

        self.try_write().await
    }

    pub fn server_urls(&self) -> Vec<&XRayServer> {
        self.server_urls.iter().collect()
    }

    pub fn remotes(&self) -> Vec<&Remote> {
        self.remotes.iter().collect()
    }

    pub(crate) async fn read_or_default() -> Self {
        FileMan::read_data(DEFAULT_CONFIG_FILE_NAME)
            .await
            .ok()
            .and_then(|v| serde_json::from_slice::<Self>(v.as_ref()).ok())
            .unwrap_or_default()
    }

    pub(crate) async fn try_write(&self) -> anyhow::Result<()> {
        let serialized = serde_json::to_string_pretty(self)?;
        FileMan::save_data(DEFAULT_CONFIG_FILE_NAME, serialized).await?;
        Ok(())
    }
}
