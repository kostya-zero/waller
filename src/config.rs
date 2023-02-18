use std::path::Path;
use crate::paths::Paths;
use std::fs;

#[derive(Debug, serde::Deserialize)]
pub enum ApplyMethod {
    swaybg,
    feh
}

#[derive(Debug, serde::Deserialize)]
pub enum ApplyMode {
    fit,
    fill,
    center,
    stretch
}

#[derive(Debug, serde::Deserialize)]
pub struct ConfigStruct {
    pub method: ApplyMethod,
    pub mode: ApplyMode,
    pub default_wall: String
}

pub struct ConfigManager;
impl ConfigManager {
    pub fn is_exists() -> bool {
        Path::new(&Paths::home_config()).exists()
    }

    pub fn get_config() -> ConfigStruct {
        let content = fs::read_to_string(Paths::home_config()).expect("Failed to read config file.");
        let toml = toml::from_str(&content).unwrap();
        return toml;
    }
}
