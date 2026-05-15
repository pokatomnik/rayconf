use std::path::PathBuf;

pub(crate) struct FileMan;

static USER_CONFIG_DIR_NAME: &'static str = ".config";
static RAYCONF_DIR_NAME: &'static str = "rayconf";

impl FileMan {
    fn resolve_rayconf_dir_path() -> Option<PathBuf> {
        let homedir = std::env::home_dir()?;
        let dir = homedir.join(USER_CONFIG_DIR_NAME).join(RAYCONF_DIR_NAME);
        Some(dir)
    }

    async fn ensure_rayconf_dir_exists() -> anyhow::Result<()> {
        let rayconf_dir_path = Self::resolve_rayconf_dir_path()
            .ok_or_else(|| anyhow::anyhow!("Cannot resolve user home directory"))?;
        tokio::fs::create_dir_all(rayconf_dir_path).await?;
        Ok(())
    }

    pub async fn save_data(fname: impl AsRef<str>, data: impl AsRef<[u8]>) -> anyhow::Result<()> {
        Self::ensure_rayconf_dir_exists().await?;
        let rayconf_dir_path = Self::resolve_rayconf_dir_path()
            .ok_or_else(|| anyhow::anyhow!("Cannot resolve user home directory"))?;
        let full_file_path = rayconf_dir_path.join(fname.as_ref());
        tokio::fs::write(full_file_path, data).await?;
        Ok(())
    }

    pub async fn read_data(fname: impl AsRef<str>) -> anyhow::Result<impl AsRef<[u8]>> {
        let rayconf_dir_path = Self::resolve_rayconf_dir_path()
            .ok_or_else(|| anyhow::anyhow!("Cannot resolve user home directory"))?;
        let full_file_path = rayconf_dir_path.join(fname.as_ref());
        let data = tokio::fs::read(full_file_path).await?;
        Ok(data)
    }
}
