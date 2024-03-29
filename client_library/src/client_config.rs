use std::{fs, path::Path};

use serde::{Deserialize, Serialize};
use toml::Table;

use crate::client_types::types::{DeviceCapabilityStatus, DeviceCapabilityType};

#[derive(Serialize, Deserialize)]
pub struct ClientConfig {
    pub server_ip: Option<String>,
    pub device_name: String,
    pub capability: Table,
}

#[derive(Clone)]
pub struct ParsedConfig {
    pub server_ip: Option<String>,
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
                        eprintln!("Capability {name} in config file has invalid fields, expected a boolean for available field");
                        false
                    }
                };

                let capa_type = match value.get("type") {
                    Some(v) => v.clone(),
                    None => {
                        eprintln!("Capability {name} in config file is missing a type, defaulting to button");
                        toml::Value::Boolean(false)
                    }
                };

                let capa_type = match capa_type {
                    toml::Value::String(v) => v,
                    _ => String::from("button")
                };

                let capa_type = match &*capa_type {
                    "button" => DeviceCapabilityType::Button,
                    "slider" => DeviceCapabilityType::Slider,
                    _ => {
                        eprintln!("Unrecognized capability type for capability {name}, defaulting to button");
                        DeviceCapabilityType::Button
                    }
                };

                DeviceCapabilityStatus {
                    capability: name,
                    available,
                    r#type: capa_type as i32,
                    value: None
                }
            }).collect();

            return Ok(ParsedConfig {
                server_ip: config.server_ip,
                device_name: config.device_name,
                capabilities,
            });
        }

        println!("No config found, generating a default config");
        let config = ClientConfig::default();
        let toml = toml::to_string(&config)?;

        fs::write(config_path, toml)?;
        return Ok(ParsedConfig {
            server_ip: None,
            device_name: config.device_name,
            capabilities: Vec::new(),
        });
    }
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            server_ip: Some(String::from(
                "Your server IP here, or nothing if you want to set it programmatically",
            )),
            device_name: String::from("Unnamed Device"),
            capability: Table::new(),
        }
    }
}

impl Default for ParsedConfig {
    fn default() -> Self {
        Self {
            server_ip: None,
            device_name: String::from("Unnamed Device"),
            capabilities: Vec::new(),
        }
    }
}
