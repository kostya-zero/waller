use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct Collection {
    collection: Vec<Wallpaper>,
}

pub enum CollectionError {
    FileNotFound,
    IndexOutOfRange,
    NotFound,
}

impl Collection {
    pub fn is_empty(&self) -> bool {
        self.collection.is_empty()
    }

    pub fn get_by_name(&self, name: &str) -> Result<Wallpaper, CollectionError> {
        for wall in self.collection.clone() {
            if wall.name == name {
                return Ok(wall);
            }
        }
        Err(CollectionError::NotFound)
    }

    pub fn get_by_index(&self, index: usize) -> Result<Wallpaper, CollectionError> {
        if index > self.collection.len() {
            return Err(CollectionError::IndexOutOfRange);
        }

        let wallpaper = self.collection[index].clone();

        if !Path::new(&wallpaper.path).exists() {
            return Err(CollectionError::FileNotFound);
        }

        Ok(wallpaper)
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Wallpaper {
    name: String,
    path: String,
}

impl Wallpaper {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }
}
