use crate::managment::CargoConfig;
use anyhow::{Ok, Result};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct AddCommand {
    #[clap(long = "file", short = 'f', default_value = "config", hide = true)]
    pub file_path: PathBuf,

    pub source: String,

    pub registry: String,
}

impl AddCommand {
    pub async fn run(&self) -> Result<()> {
        let path = PathBuf::from(env!("CARGO_HOME")).join(&self.file_path);
        let mut cargo_config = CargoConfig::load(path).await?;
        cargo_config
            .add_source(&self.source, &self.registry)
            .await?;
        println!("\n Add source successfully.\n");
        Ok(())
    }
}
