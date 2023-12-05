use std::{env, fs, path::Path, process::exit};

use crate::{
    collection::Collection,
    config::{ApplyMethod, Config},
    term::Term,
};

pub enum ManagerError {
    LoadError(String),
    WriteError(String),
}

pub struct Manager;
impl Manager {
    pub fn is_exists() -> bool {
        if !Path::new(&Self::get_config_path()).exists() {
            return false;
        }

        if !Path::new(&Self::get_collection_path()).exists() {
            return false;
        }

        true
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

    pub fn get_collection_path() -> String {
        home::home_dir()
            .expect("Failed to get home directory")
            .display()
            .to_string()
            + "/.config/waller/collection.json"
    }

    pub fn load_config() -> Result<Config, ManagerError> {
        if let Ok(content) = fs::read_to_string(Self::get_config_path()) {
            match toml::from_str::<Config>(&content) {
                Ok(i) => Ok(i),
                Err(_) => Err(ManagerError::LoadError(String::from(
                    "Configuration file is bad formatted.",
                ))),
            }
        } else {
            Err(ManagerError::LoadError(String::from(
                "Cannot read configuration file.",
            )))
        }
    }

    pub fn load_collection() -> Result<Collection, ManagerError> {
        if let Ok(content) = fs::read_to_string(Self::get_collection_path()) {
            match serde_json::from_str::<Collection>(&content) {
                Ok(i) => Ok(i),
                Err(_) => Err(ManagerError::LoadError(String::from(
                    "Collection file is bad formatted.",
                ))),
            }
        } else {
            Err(ManagerError::LoadError(String::from(
                "Cannot read collection file.",
            )))
        }
    }

    pub fn write_config(conf: Config) -> Result<(), ManagerError> {
        if let Ok(content) = toml::to_string(&conf) {
            match fs::write(Self::get_config_path(), content) {
                Ok(_) => Ok(()),
                Err(_) => Err(ManagerError::WriteError(String::from(
                    "Cannot write configuration content to the file.",
                ))),
            }
        } else {
            Err(ManagerError::WriteError(String::from(
                "Failed to format configuration struct to string.",
            )))
        }
    }

    pub fn write_collection(collection: Collection) -> Result<(), ManagerError> {
        if let Ok(content) = serde_json::to_string(&collection) {
            match fs::write(Self::get_collection_path(), content) {
                Ok(_) => Ok(()),
                Err(_) => Err(ManagerError::WriteError(String::from(
                    "Cannot write collection content to the file.",
                ))),
            }
        } else {
            Err(ManagerError::WriteError(String::from(
                "Failed to format collection struct to string.",
            )))
        }
    }

    pub fn make_default_config() {
        if !Path::new(&Self::get_config_dir()).exists() {
            let result_dir = fs::create_dir(Self::get_config_dir());
            if result_dir.is_err() {
                Term::fatal("Failed to create directory for waller configuration file.");
                exit(1);
            }
        }

        if !Path::new(&Self::get_config_path()).exists() {
            let mut construct = Config::default();
            if env::var("XDG_CURRENT_DESKTOP").is_ok() {
                let desktop: &str = env!("XDG_CURRENT_DESKTOP", "XDG_CURRENT_DESKTOP not set!");
                match desktop {
                    "GNOME" => construct.method = Some(ApplyMethod::gnome),
                    "sway" => construct.method = Some(ApplyMethod::swaybg),
                    "Hyprland" => construct.method = Some(ApplyMethod::swaybg),
                    "KDE" => construct.method = Some(ApplyMethod::kde),
                    _ => construct.method = Some(ApplyMethod::feh),
                }
            }
            let result_file = fs::write(
                Self::get_config_path(),
                toml::to_string(&construct).expect("Failed to format construct to string."),
            );
            if result_file.is_err() {
                Term::fatal("Failed to write content to configuration file!");
                exit(1);
            }
        }

        if !Path::new(&Self::get_collection_path()).exists() {
            let default_collection: Collection = Collection::default();
            let result_file = fs::write(
                Self::get_collection_path(),
                serde_json::to_string(&default_collection)
                    .expect("Failed to format construct to string."),
            );
            if result_file.is_err() {
                Term::fatal("Failed to write content to collection file!");
                exit(1);
            }
        }
    }
}
