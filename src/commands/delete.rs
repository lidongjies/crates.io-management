use crate::managment::CargoConfig;
use anyhow::{Ok, Result};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct DeleteCommand {
    #[clap(long = "file", short = 'f', default_value = "config", hide = true)]
    pub file_path: PathBuf,

    pub source: String,
}

impl DeleteCommand {
    pub async fn run(&self) -> Result<()> {
        let path = PathBuf::from(env!("CARGO_HOME")).join(&self.file_path);
        let mut cargo_config = CargoConfig::load(path).await?;
        cargo_config.delete_source(&self.source).await?;
        println!("\n Delete source {:} successfully.\n", &self.source);
        Ok(())
    }
}
