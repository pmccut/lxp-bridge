use crate::prelude::*;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub inverters: Vec<Inverter>,
    pub mqtt: Mqtt,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Inverter {
    pub host: String,
    pub port: u16,
    pub serial: String,
    #[serde(deserialize_with = "de_datalog")]
    pub datalog: Datalog,
}

fn de_datalog<'de, D>(deserializer: D) -> Result<Datalog, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let raw = String::deserialize(deserializer)?;

    Ok(Datalog::from_str(&raw))
}

#[derive(Debug, Deserialize)]
pub struct Mqtt {
    pub host: String,
    #[serde(default = "Config::default_mqtt_port")]
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,

    #[serde(default = "Config::default_mqtt_namespace")]
    pub namespace: String,
}

impl Config {
    pub fn new(file: String) -> Result<Self> {
        let content = std::fs::read_to_string(file)?;
        let config = serde_yaml::from_str(&content)?;

        Ok(config)
    }

    fn default_mqtt_port() -> u16 {
        1883
    }
    fn default_mqtt_namespace() -> String {
        "lxp".to_string()
    }
}
