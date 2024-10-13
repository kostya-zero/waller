use std::{
    env,
    path::{Path, PathBuf},
};

pub struct Platform;

pub enum PlatformName {
    Linux,
    Unknown,
}

impl Platform {
    pub fn get_config_path() -> PathBuf {
        Self::get_config_dir_path().join("config.toml")
    }

    pub fn get_collection_path() -> PathBuf {
        Self::get_config_dir_path().join("collection.dat")
    }

    pub fn get_config_dir_path() -> PathBuf {
        let user_home = Self::get_user_home();
        user_home.join(".config").join("waller")
    }

    pub fn get_platform() -> PlatformName {
        match env::consts::OS {
            "linux" => PlatformName::Linux,
            _ => PlatformName::Unknown,
        }
    }

    pub fn get_user_home() -> PathBuf {
        Path::new(&env::var("HOME").unwrap()).to_path_buf()
    }

    pub fn check_config_exists() -> bool {
        Path::new(Self::get_config_path().as_path()).exists()
    }

    pub fn check_collection_exists() -> bool {
        Path::new(Self::get_collection_path().as_path()).exists()
    }
}
