use crate::managment::CargoConfig;
use anyhow::{Ok, Result};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct ListCommand {
    #[clap(long = "file", short = 'f', default_value = "config", hide = true)]
    pub file_path: PathBuf,
}

impl ListCommand {
    pub async fn run(&self) -> Result<()> {
        let path = PathBuf::from(env!("CARGO_HOME")).join(&self.file_path);
        let cargo_config = CargoConfig::load(path).await?;
        let sources = cargo_config.config.source.unwrap().into_keys();
        let output_sources: Vec<String> = sources.collect();
        println!("\n {:} \n", output_sources.join("\n"));
        Ok(())
    }
}
