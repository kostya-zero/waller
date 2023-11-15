use crate::term::Term;
use serde::{Deserialize, Serialize};
use std::fs;
use std::{env, path::Path, process::exit};

#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ApplyMethod {
    swaybg,
    feh,
    gnome,
    kde,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ApplyMode {
    fit,
    fill,
    center,
    stretch,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub method: Option<ApplyMethod>,
    pub mode: Option<ApplyMode>,
    pub walls: Option<Vec<String>>,
    pub recent: Option<String>,
}

pub struct Manager;
impl Manager {
    pub fn is_exists() -> bool {
        Path::new(&Self::get_config_path()).exists()
    }

    pub fn get_config_path() -> String {
        home::home_dir()
            .expect("Failed to get home directory")
            .display()
            .to_string()
            + "/.config/waller/config.toml"
    }

    pub fn get_config_dir() -> String {
        home::home_dir()
            .expect("Failed to get home directory")
            .display()
            .to_string()
            + "/.config/waller"
    }

    pub fn get_config() -> Config {
        let content =
            fs::read_to_string(Self::get_config_path()).expect("Failed to read config file.");
        toml::from_str::<Config>(&content)
            .expect("Failed to deserialize configuration file. Some fields might be missing.")
    }

    pub fn write_config(conf: Config) {
        let content = toml::to_string(&conf).expect("Error");
        fs::write(Self::get_config_path(), content).expect("Failed to write config file.");
    }

    pub fn make_default_config() {
        let mut construct = Config {
            method: Some(ApplyMethod::swaybg),
            mode: Some(ApplyMode::center),
            walls: Some(vec![]),
            recent: Some("".to_string()),
        };

        if env::var("XDG_CURRENT_DESKTOP").is_ok() {
            let desktop: &str = env!("XDG_CURRENT_DESKTOP", "XDG_CURRENT_DESKTOP not set!");
            match desktop {
                "GNOME" => construct.method = Some(ApplyMethod::gnome),
                "sway" => construct.method = Some(ApplyMethod::swaybg),
                "KDE" => construct.method = Some(ApplyMethod::kde),
                _ => construct.method = Some(ApplyMethod::feh),
            }
        }

        if !Path::new(&Self::get_config_dir()).exists() {
            let result_dir = fs::create_dir(Self::get_config_dir());
            if result_dir.is_err() {
                Term::fatal("Failed to create directory for waller configuration file.");
                exit(1);
            }
        }

        if !Path::new(&Self::get_config_path()).exists() {
            let result_file = fs::write(
                Self::get_config_path(),
                toml::to_string(&construct).expect("Failed to format construct to string."),
            );
            if result_file.is_err() {
                Term::fatal("Failed to write content to configuration file!");
                exit(1);
            }
        }
    }
}
