use std::{fs, path::Path};

use serde::{Deserialize, Serialize};
use toml::Table;

use crate::client_types::types::DeviceCapabilityStatus;

#[derive(Serialize, Deserialize)]
pub struct ClientConfig {
    pub device_name: String,
    pub capability: Table,
}

pub struct ParsedConfig {
    pub device_name: String,
    pub capabilities: Vec<DeviceCapabilityStatus>,
}

impl ClientConfig {
    pub fn load_config(path: &str) -> anyhow::Result<ParsedConfig> {
        let config_path = Path::new(path);

        if config_path.exists() {
            let content = fs::read_to_string(config_path)?;
            let config: ClientConfig = toml::from_str(&content)?;

            let capabilities = config.capability.into_iter().map(|(name, value)|{
                let value = match value {
                    toml::Value::Table(v) => v,
                    _ => {
                        println!("capability {name} in config file has invalid fields, expected a table as contents");
                        Table::new()
                    }
                };

                let available = match value.get("available") {
                    Some(v) => v.clone(),
                    None => toml::Value::Boolean(false)
                };

                let available = match available {
                    toml::Value::Boolean(v) => v,
                    _ => {
                        println!("capability {name} in config file has invalid fields, expected a boolean for available field");
                        false
                    }
                };
                return DeviceCapabilityStatus {
                    capability: name,
                    available
                }
            }).collect();

            return Ok(ParsedConfig {
                device_name: config.device_name,
                capabilities,
            });
        }

        let config = ClientConfig::default();
        let toml = toml::to_string(&config)?;

        fs::write(config_path, toml)?;
        return Ok(ParsedConfig {
            device_name: config.device_name,
            capabilities: Vec::new(),
        });
    }
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            device_name: String::from("Unnamed Device"),
            capability: Table::new(),
        }
    }
}

impl Default for ParsedConfig {
    fn default() -> Self {
         Self {
             device_name: String::from("Unnamed Device"),
             capabilities: Vec::new(),
         }
    }
}
