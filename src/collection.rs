use serde::{Deserialize, Serialize};
use std::path::Path;

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

    pub fn is_exists(&self, name: &str) -> bool {
        for item in self.collection.clone().iter() {
            if item.name == name {
                return true;
            }
        }
        false
    }

    pub fn get_collection(&self) -> Vec<Wallpaper> {
        self.collection.clone()
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

    pub fn get_index(&self, name: &str) -> Result<usize, CollectionError> {
        for (index, item) in self.collection.clone().iter().enumerate() {
            if item.name == name {
                return Ok(index);
            }
        }
        Err(CollectionError::NotFound)
    }

    pub fn add(&mut self, new_image: Wallpaper) {
        self.collection.push(new_image);
    }

    pub fn remove_by_index(&mut self, index: usize) -> Result<(), CollectionError> {
        if index > self.collection.len() {
            return Err(CollectionError::IndexOutOfRange);
        }

        self.collection.remove(index);
        Ok(())
    }
}

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct Wallpaper {
    name: String,
    path: String,
}

impl Wallpaper {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn set_path(&mut self, path: &str) {
        self.path = path.to_string();
    }
}
