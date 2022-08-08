use crate::cargo_config::CargoConfig;
use anyhow::{bail, Result};
use clap::Parser;
use log::debug;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct SetCommand {
    source: String,

    #[clap(long = "file", short = 'f', default_value = "config", hide = true)]
    pub file_path: PathBuf,
}

impl SetCommand {
    pub async fn run(&self) -> Result<()> {
        let path = PathBuf::from(env!("CARGO_HOME")).join(&self.file_path);
        debug!("Loading config from {}", path.display());
        let mut cargo_config = CargoConfig::load(path).await?;
        match cargo_config.config.source.contains_key(&self.source) {
            false => bail!("target source {:} not exists", self.source),
            true => cargo_config.update_source(&self.source).await?,
        };
        Ok(())
    }
}
