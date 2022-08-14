use crate::managment::CargoConfig;
use crate::managment::DEFAULT_CARGO_CONFIG_NAME;
use anyhow::{bail, Ok, Result};
use clap::Parser;
use log::debug;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct SetCommand {
    /// cargo config file dir
    #[clap(long = "dir", short = 'd', default_value = env!("CARGO_HOME"))]
    pub dir_path: PathBuf,

    /// custom source name
    pub source: String,
}

impl SetCommand {
    pub async fn run(&self) -> Result<()> {
        let path = PathBuf::from(&self.dir_path).join(DEFAULT_CARGO_CONFIG_NAME);
        debug!("Loading config from {}", path.display());
        let mut cargo_config = CargoConfig::load(path).await?;
        // TODO：找出跟 self.source 相似的源名称，提示用户
        match cargo_config
            .config
            .source
            .as_ref()
            .unwrap()
            .contains_key(&self.source)
        {
            false => bail!("target source {:} not exists", self.source),
            true => cargo_config.update_source(&self.source).await?,
        };
        println!("Use {:} successful.", self.source);
        Ok(())
    }
}
