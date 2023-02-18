use std::path::Path;
use crate::paths::Paths;
use std::fs;

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub enum ApplyMethod {
    swaybg,
    feh
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub enum ApplyMode {
    fit,
    fill,
    center,
    stretch
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct ConfigStruct {
    pub method: ApplyMethod,
    pub mode: ApplyMode,
    pub default_wall: String,
    pub random_folder: String
}

pub struct ConfigManager;
impl ConfigManager {
    pub fn is_exists() -> bool {
        Path::new(&Paths::home_config()).exists()
    }

    pub fn get_config() -> ConfigStruct {
        let content = fs::read_to_string(Paths::home_config()).expect("Failed to read config file.");
        let toml = toml::from_str(&content).expect("Failed to deserialize configuration file.");
        return toml;
    }

    pub fn make_default_config() {
        let construct = ConfigStruct {
            method: ApplyMethod::swaybg,
            mode: ApplyMode::center,
            default_wall: "".to_string(),
            random_folder: "".to_string()
        };

        if !Path::new(&Paths::home_config_dir()).exists() {
            fs::create_dir(&Paths::home_config_dir());
        }

        if !Path::new(&Paths::home_config()).exists() {
            fs::write(&Paths::home_config(), toml::to_string(&construct).expect("Failed to format construct to string."));
        }
    }
}
