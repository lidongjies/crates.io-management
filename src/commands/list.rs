use crate::managment::CargoConfig;
use crate::managment::DEFAULT_CARGO_CONFIG_NAME;
use anyhow::{Ok, Result};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct ListCommand {
    /// cargo config file dir
    #[clap(long = "dir", short = 'd', default_value = env!("CARGO_HOME"))]
    pub dir_path: PathBuf,
}

impl ListCommand {
    pub async fn run(&self) -> Result<()> {
        let path = PathBuf::from(&self.dir_path).join(DEFAULT_CARGO_CONFIG_NAME);
        let cargo_config = CargoConfig::load(path).await?;
        let sources = cargo_config.config.source.unwrap().into_keys();
        let output_sources: Vec<String> = sources.collect();
        println!("\n {:} \n", output_sources.join("\n"));
        Ok(())
    }
}
