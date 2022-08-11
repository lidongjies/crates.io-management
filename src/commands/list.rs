use crate::managment::CargoConfig;
use anyhow::{Ok, Result};
use clap::Parser;
use log::debug;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct ListCommand {
    #[clap(long = "file", short = 'f', default_value = "config", hide = true)]
    pub file_path: PathBuf,
}

impl ListCommand {
    pub async fn run(&self) -> Result<()> {
        let path = PathBuf::from(env!("CARGO_HOME")).join(&self.file_path);
        debug!("Loading config from {}", path.display());
        let cargo_config = CargoConfig::load(path).await?;
        let sources = cargo_config.config.source.as_ref().unwrap().keys();
        debug!("Sources: {:?}", sources);
        sources.for_each(|source| println!("{:}", source));
        Ok(())
    }
}
