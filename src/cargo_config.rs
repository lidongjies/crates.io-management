use anyhow::Result;
use serde_derive::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug)]
pub struct SourceManagement {}

#[derive(Deserialize, Serialize, Debug)]
pub struct Source {
    pub registry: Option<String>,
    pub replace_with: Option<String>,
    pub directory: Option<String>,
    pub local_registry: Option<String>,
    pub git: Option<String>,
    pub branch: Option<String>,
    pub tag: Option<String>,
    pub rev: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub source: HashMap<String, Source>,
}

#[derive(Debug)]
pub struct CargoConfig {
    pub config: Config,
    pub config_path: PathBuf,
    pub source_management: SourceManagement,
}

impl CargoConfig {
    pub async fn load(config_path: PathBuf) -> Result<CargoConfig> {
        let toml = tokio::fs::read_to_string(&config_path).await?;
        let config: Config = toml::from_str(&toml)?;
        Ok(Self {
            config,
            config_path: config_path,
            source_management: SourceManagement {},
        })
    }

    pub async fn update_source(&mut self, source_name: &str) -> Result<()> {
        let mut source = self.config.source.get_mut("crates-io").unwrap();
        source.replace_with = Some(source_name.to_string());
        let contents = toml::to_vec(&self.config).unwrap();
        tokio::fs::write(&self.config_path, &contents).await?;
        Ok(())
    }
}
