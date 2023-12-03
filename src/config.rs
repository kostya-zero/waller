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

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Config {
    pub method: Option<ApplyMethod>,
    pub mode: Option<ApplyMode>,
    pub recent: Option<String>,
}


