use crate::managment::CargoConfig;
use crate::managment::DEFAULT_CARGO_CONFIG_NAME;
use anyhow::{Ok, Result};
use clap::Parser;
use log::debug;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct CurrentCommand {
    /// cargo config file dir
    #[clap(long = "dir", short = 'd', default_value = env!("CARGO_HOME"))]
    pub dir_path: PathBuf,
}

impl CurrentCommand {
    pub async fn run(&self) -> Result<()> {
        let path = PathBuf::from(&self.dir_path).join(DEFAULT_CARGO_CONFIG_NAME);
        debug!("Loading config from {}", path.display());
        let cargo_config = CargoConfig::load(path).await?;
        let source = cargo_config
            .config
            .source
            .as_ref()
            .unwrap()
            .get("crates-io");
        println!(
            "\n\tCurrent source is {:}.\n",
            source.unwrap().replace_with.as_ref().unwrap()
        );
        Ok(())
    }
}
