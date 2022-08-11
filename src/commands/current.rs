use crate::managment::CargoConfig;
use anyhow::{Ok, Result};
use clap::Parser;
use log::debug;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct CurrentCommand {
    #[clap(long = "file", short = 'f', default_value = "config", hide = true)]
    pub file_path: PathBuf,
}

impl CurrentCommand {
    pub async fn run(&self) -> Result<()> {
        let path = PathBuf::from(env!("CARGO_HOME")).join(&self.file_path);
        debug!("Loading config from {}", path.display());
        let cargo_config = CargoConfig::load(path).await?;
        let source = cargo_config
            .config
            .source
            .as_ref()
            .unwrap()
            .get("crates-io");
        println!(
            "\n Current source is {:}. \n",
            source.unwrap().replace_with.as_ref().unwrap()
        );
        Ok(())
    }
}
