use crate::entities::remote::Remote;
use crate::entities::remote_decoder::RemoteDecoder;
use crate::entities::xray_server::XRayServer;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::PathBuf;

static DEFAULT_CONFIG_FILE_NAME: &'static str = ".rayconf.json";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct Config {
    #[serde(rename = "serverURLs")]
    server_urls: Vec<XRayServer>,

    #[serde(rename = "remotes")]
    remotes: Vec<Remote>,
}

impl Config {
    pub fn add_local(&mut self, url: impl AsRef<str>) -> anyhow::Result<()> {
        let url = XRayServer::try_from(url.as_ref().to_owned())?;
        self.server_urls.push(url);

        self.try_write()
    }

    pub fn add_remote(
        &mut self,
        name: impl AsRef<str>,
        url: impl AsRef<str>,
        decoder: impl AsRef<RemoteDecoder>,
    ) -> anyhow::Result<()> {
        let remote = Remote::new(name.as_ref(), url.as_ref(), decoder.as_ref().clone());

        self.remotes.push(remote);

        self.try_write()
    }

    pub fn remove_locals_by_indexes(&mut self, indexes: Vec<usize>) -> anyhow::Result<()> {
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

        self.try_write()
    }

    pub fn remove_remote_by_indexes(&mut self, indexes: Vec<usize>) -> anyhow::Result<()> {
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

        self.try_write()
    }

    pub fn server_urls(&self) -> Vec<&XRayServer> {
        self.server_urls.iter().collect()
    }

    pub fn remotes(&self) -> Vec<&Remote> {
        self.remotes.iter().collect()
    }

    fn home_dir() -> Option<PathBuf> {
        std::env::home_dir()
    }

    fn get_config_path() -> anyhow::Result<PathBuf> {
        let home_dir = Self::home_dir().ok_or_else(|| anyhow::Error::msg(""))?;
        Ok(home_dir.join(DEFAULT_CONFIG_FILE_NAME))
    }

    pub(crate) fn read_or_default() -> Self {
        let Ok(config_path) = Self::get_config_path() else {
            return Self::default();
        };

        let Ok(config_str) = std::fs::read_to_string(config_path) else {
            return Self::default();
        };

        serde_json::from_str(&config_str).unwrap_or_else(|_| Self::default())
    }

    pub(crate) fn try_write(&self) -> anyhow::Result<()> {
        let serialized = serde_json::to_string_pretty(self)?;
        Ok(std::fs::write(Self::get_config_path()?, serialized)?)
    }
}
