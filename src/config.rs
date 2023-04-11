use std::{path::Path, process::exit, env};
use crate::{paths::Paths, term::Term};
use serde::{Deserialize, Serialize};
use std::fs;

#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ApplyMethod {
    swaybg,
    feh,
    gnome
}

#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ApplyMode {
    fit,
    fill,
    center,
    stretch
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConfigStruct {
    pub method: ApplyMethod,
    pub mode: ApplyMode,
    pub walls: Vec<String>
}

pub struct ConfigManager;
impl ConfigManager {
    pub fn is_exists() -> bool {
        Path::new(&Paths::home_config()).exists()
    }

    pub fn get_config() -> ConfigStruct {
        let content = fs::read_to_string(Paths::home_config()).expect("Failed to read config file.");
        let toml = toml::from_str::<ConfigStruct>(&content).expect("Failed to deserialize configuration file.");
        return toml;
    }

    pub fn write_config(conf: ConfigStruct) {
        let content = toml::to_string(&conf).expect("Error");
        fs::write(Paths::home_config(), content).expect("Failed to write config file.");
    }

    pub fn make_default_config() {
        let mut construct = ConfigStruct {
            method: ApplyMethod::swaybg,
            mode: ApplyMode::center,
            walls: vec![]
        };

        if env::var("XDG_CURRENT_DESKTOP").is_ok() {
            let desktop: &str = env!("XDG_CURRENT_DESKTOP", "XDG_CURRENT_DESKTOP not set!");
            match desktop {
                "GNOME" => construct.method = ApplyMethod::gnome,
                "sway" => construct.method = ApplyMethod::swaybg,
                _ => construct.method = ApplyMethod::feh
            }
        }

        if !Path::new(&Paths::home_config_dir()).exists() {
            let result_dir = fs::create_dir(&Paths::home_config_dir());
            if  result_dir.is_err() {
                Term::fatal("Failed to create directory for waller configuration file.".to_string());
                exit(1);
            }
        }

        if !Path::new(&Paths::home_config()).exists() {
            let result_file = fs::write(&Paths::home_config(), toml::to_string(&construct).expect("Failed to format construct to string."));
            if result_file.is_err() {
                Term::fatal("Failed to write content to configuration file!".to_string());
                exit(1);
            }
        }
    }
}
