use crate::managment::CargoConfig;
use crate::managment::DEFAULT_CARGO_CONFIG_NAME;
use anyhow::{Ok, Result};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct AddCommand {
    /// cargo config file dir
    #[clap(long = "dir", short = 'd', default_value = env!("CARGO_HOME"))]
    pub dir_path: PathBuf,

    /// custom source name
    pub source: String,

    /// custom source registry
    pub registry: String,
}

impl AddCommand {
    pub async fn run(&self) -> Result<()> {
        let path = PathBuf::from(&self.dir_path).join(DEFAULT_CARGO_CONFIG_NAME);
        let mut cargo_config = CargoConfig::load(path).await?;
        cargo_config
            .add_source(&self.source, &self.registry)
            .await?;
        println!("\n Add source successfully.\n");
        Ok(())
    }
}
