use std::fs;
use std::path::Path;

use toml;
use crate::config::MainConfig;

pub fn load_toml_config<P: AsRef<Path>>(path: P) -> Result<MainConfig, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let config: MainConfig = toml::from_str(&content)?;
    Ok(config)
}
