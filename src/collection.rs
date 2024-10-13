use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::platform::Platform;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Collection {
    array: Vec<Wallpaper>,
    recent: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Wallpaper {
    path: PathBuf,
    name: String,
}

impl Wallpaper {
    pub fn new(path: PathBuf, name: String) -> Wallpaper {
        Wallpaper { path, name }
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

impl Collection {
    pub fn new() -> Collection {
        Collection::default()
    }

    pub fn load() -> Result<Collection> {
        if let Ok(content) = fs::read_to_string(Platform::get_collection_path()) {
            let collection: Collection = bincode::deserialize(content.as_bytes())?;
            Ok(collection)
        } else {
            Err(anyhow!("Failed to load collection from disk."))
        }
    }

    pub fn save(&self) -> Result<()> {
        let dir_path = Platform::get_config_dir_path();
        if !Path::new(&dir_path).exists() {
            fs::create_dir(&dir_path)?;
        }
        let content = bincode::serialize(&self)?;
        fs::write(Platform::get_collection_path(), content)?;
        Ok(())
    }

    pub fn add(&mut self, path: PathBuf, name: String) {
        self.array.push(Wallpaper::new(path, name));
    }

    pub fn get(&self, name: &str) -> Option<&Wallpaper> {
        self.array.iter().find(|c| c.get_name() == name)
    }

    pub fn remove(&mut self, name: &str) {
        self.array.retain(|c| c.get_name() != name);
    }

    pub fn get_array(&self) -> &Vec<Wallpaper> {
        &self.array
    }

    pub fn get_recent(&self) -> &str {
        &self.recent
    }

    pub fn set_recent(&mut self, name: &str) {
        self.recent = name.to_owned();
    }
}
