use crate::collection::Wallpaper;
use serde::{Deserialize, Serialize};

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

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct Config {
    pub method: Option<ApplyMethod>,
    pub mode: Option<ApplyMode>,
    pub recent: Option<Wallpaper>,
}

impl Config {
    pub fn get_method(&self) -> Option<ApplyMethod> {
        self.method.clone()
    }

    pub fn get_mode(&self) -> Option<ApplyMode> {
        self.mode.clone()
    }

    pub fn get_recent(&self) -> Option<Wallpaper> {
        self.recent.clone()
    }

    pub fn set_recent(&mut self, wallpaper: Wallpaper) {
        self.recent = Some(wallpaper);
    }
}

