use std::{path::Path, process::exit, env};
use crate::{paths::Paths, term::Term};
use std::fs;

#[allow(non_camel_case_types)]
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub enum ApplyMethod {
    swaybg,
    feh,
    gnome
}

#[allow(non_camel_case_types)]
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub enum ApplyMode {
    fit,
    fill,
    center,
    stretch
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct ConfigStruct {
    pub method: ApplyMethod,
    pub mode: ApplyMode,
    pub random_folder: Option<String>
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

    pub fn make_default_config() {
        let mut construct = ConfigStruct {
            method: ApplyMethod::swaybg,
            mode: ApplyMode::center,
            random_folder: Some("".to_string())
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
        
        if !Path::new(&Paths::home_config_walls()).exists() {
            fs::write(&Paths::home_config_walls(), "").expect("Failed to create blank text file.")
        }
    }

    pub fn get_walls() -> Vec<String> {
        let contents: String = fs::read_to_string(Paths::home_config_walls()).expect("Failed to read file.");
        let lines = contents.lines().collect::<Vec<_>>();
        let mut string_lines: Vec<String> = Vec::new();
        for line in lines {
            string_lines.push(line.trim().to_string());
        }
        return string_lines;
    }

    pub fn write_walls(walls: Vec<String>) {
       let mut new_content: String = "".to_string();
       for wall in walls {
           new_content += &(wall.to_string() + &"\n".to_string());
       }
       let result_file = fs::write(Paths::home_config_walls(), new_content);
       if result_file.is_err() {
           Term::fatal("Failed to update configuration file.".to_string());
           exit(1);
       }
    }

}
