use anyhow::{self, Context, Result};
use std::{env, fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::platform::Platform;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    method: Option<Method>,
    fit: Option<Fit>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Method {
    swaybg,
    feh,
    hyprpaper,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum Fit {
    #[default]
    Fit,
    Fill,
    Center,
    Stretch,
}

impl Default for Config {
    fn default() -> Self {
        let desktop = env::var("XDG_CURRENT_DESKTOP");

        let method = if let Ok(desktop_name) = desktop {
            match desktop_name.as_str() {
                "sway" => Some(Method::swaybg),
                "Hyprland" => Some(Method::hyprpaper),
                _ => Some(Method::feh),
            }
        } else {
            Some(Method::feh)
        };

        Self {
            method,
            fit: Some(Fit::Fit),
        }
    }
}

impl Config {
    pub fn new() -> Config {
        Config::default()
    }

    pub fn load() -> Result<Self> {
        let content = fs::read_to_string(Platform::get_config_path())?;
        toml::from_str::<Config>(&content).context("Failed to parse configuration file.")
    }

    pub fn save(&self) -> Result<()> {
        let dir_path = Platform::get_config_dir_path();
        if !Path::new(&dir_path).exists() {
            fs::create_dir(&dir_path)?;
        }
        let content = toml::to_string(self)?;
        fs::write(Platform::get_config_path(), content)?;
        Ok(())
    }
}
