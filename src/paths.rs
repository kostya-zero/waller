use home;

pub struct Paths;
impl Paths {
    pub fn home_config() -> String {
        home::home_dir().expect("Failed to get home directory").display().to_string() + &"/.config/waller/config.toml".to_string()
    }

    pub fn home_config_dir() -> String {
        home::home_dir().expect("Failed to get home directory").display().to_string() + &"/.config/waller".to_string()
    }
    pub fn home_config_walls() -> String {
        home::home_dir().expect("Failed to get home directory").display().to_string() + &"/.config/waller/walls".to_string()
    }
}
