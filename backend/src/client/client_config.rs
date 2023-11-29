use std::{path::Path, fs};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ClientConfig {
    pub device_name: String,
}
impl ClientConfig {
    pub fn load_config(path: &str) -> anyhow::Result<Self> {
        let config_path = Path::new(path);

        if config_path.exists() {
            let content = fs::read_to_string(config_path)?;
            let config = toml::from_str(&content)?;

            return Ok(config);
        }

        let config = ClientConfig::default();
        let toml = toml::to_string(&config)?;

        fs::write(config_path, toml)?;
        return Ok(config);
    }
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            device_name: String::from("Unnamed Device"),
        }
    }
}
