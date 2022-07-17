use anyhow::Result;
use serde_derive::Deserialize;
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug)]
pub struct SourceManagement {}

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
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
        let config = toml::from_str(&toml)?;
        Ok(Self {
            config,
            config_path: config_path,
            source_management: SourceManagement {},
        })
    }
}
