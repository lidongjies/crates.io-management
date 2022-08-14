use crate::cargo_config::{Config, SourceConfig};
use anyhow::{bail, Result};
use log::{debug, info};
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub struct CargoConfig {
    pub config: Config,
    pub config_path: PathBuf,
}

const DEFAULT_SOURCES_FILE: &str = "sources.toml";
pub const DEFAULT_CARGO_CONFIG_NAME: &'static str = "config";

impl CargoConfig {
    pub async fn load(config_path: PathBuf) -> Result<CargoConfig> {
        debug!("Loading config from {}", config_path.display());

        let toml = match tokio::fs::read_to_string(&config_path).await {
            Ok(config) => config,
            Err(e) => handle_read_file_error(e, &config_path).await?,
        };

        let mut config: Config = toml::from_str(&toml)?;
        if config.source.is_none() {
            let default_source_config_path =
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(DEFAULT_SOURCES_FILE);
            let toml = tokio::fs::read_to_string(&default_source_config_path).await?;
            let default_config: Config = toml::from_str(&toml)?;
            config.source = default_config.source;
        }

        Ok(Self {
            config,
            config_path,
        })
    }

    pub async fn update_source(&mut self, source_name: &str) -> Result<()> {
        let mut source = self
            .config
            .source
            .as_mut()
            .unwrap()
            .get_mut("crates-io")
            .unwrap();
        source.replace_with = Some(source_name.to_string());
        self.save_config().await
    }

    pub async fn add_source(&mut self, source_name: &str, registry: &str) -> Result<()> {
        let mut source = SourceConfig::new();
        source.with_registry(registry.to_owned())?;
        self.config
            .source
            .as_mut()
            .unwrap()
            .insert(source_name.to_owned(), source);
        self.save_config().await
    }

    pub async fn delete_source(&mut self, source_name: &str) -> Result<()> {
        self.config.source.as_mut().unwrap().remove(source_name);
        self.save_config().await
    }

    pub async fn save_config(&self) -> Result<()> {
        let contents = toml::to_vec(&self.config).unwrap();
        tokio::fs::write(&self.config_path, &contents).await?;
        Ok(())
    }
}

async fn handle_read_file_error(e: io::Error, config_path: &PathBuf) -> Result<String> {
    match e.kind() {
        io::ErrorKind::NotFound => {
            info!("No cargo config file at {:?}", config_path);
            let default_config_path =
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(DEFAULT_SOURCES_FILE);
            info!("Default sources config at {:?}", &default_config_path);
            tokio::fs::copy(&default_config_path, config_path).await?;
            let toml = tokio::fs::read_to_string(&default_config_path).await?;
            return Ok(toml);
        }
        io::ErrorKind::PermissionDenied => {
            bail!("No permission to read cargo config")
        }
        _ => {
            bail!("Read cargo config failed");
        }
    }
}
