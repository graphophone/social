use std::fs;

use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub database: PostgresConfig,
}

impl Config {
    pub fn build(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
        let file = fs::File::open(path)?;
        let config: Config = serde_yaml::from_reader(file)?;
        Ok(config)
    }
}

#[derive(Deserialize, Debug)]
pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}