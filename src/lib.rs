use std::{path::Path, process::exit};

use anyhow::Result;
use args::build_cli;
use collection::Collection;
use config::Config;
use platform::Platform;
use terminal::Message;

mod args;
mod collection;
mod config;
mod platform;
pub mod terminal;

pub fn main() -> Result<()> {
    if !Platform::check_config_exists() {
        let config = Config::default();
        config.save()?;
    }

    if !Platform::check_collection_exists() {
        let collection = Collection::new();
        collection.save()?;
    }

    let args = build_cli();

    let mut collection = Collection::load()?;
    let config = Config::load()?;
    match args.subcommand() {
        Some(("add", sub)) => {
            if let Some(path) = sub.get_one::<String>("path") {
                let name = if let Some(name) = sub.get_one::<String>("name") {
                    name.to_owned()
                } else {
                    let file_name = Path::new(path).file_stem().unwrap().to_str().unwrap();
                    file_name.to_string()
                };

                if collection.is_exists_by_name(&name) {
                    Message::fail("The wallpaper already exists in the collection.");
                }

                collection.add(Path::new(path).to_path_buf(), name);
                collection.save()?;
                Message::done("Added wallpaper to collection.");
            } else {
                Message::fail("Please specify a path to the wallpaper.");
            }
        }
        Some(("list", _)) => {
            if collection.is_empty() {
                Message::info("Collection is empty.");
                return Ok(());
            }
            Message::list_title("Wallpapers in collection:");
            for wallpaper in collection.get_array().iter() {
                Message::item(
                    format!(
                        "{} ({})",
                        wallpaper.get_name(),
                        wallpaper.get_path().display()
                    )
                    .as_str(),
                );
            }
        }
        Some(("remove", sub)) => {
            if let Some(name) = sub.get_one::<String>("name") {
                if !collection.is_exists_by_name(name) {
                    Message::fail("The wallpaper does not exist in the collection.");
                }

                collection.remove(name)?;
                collection.save()?;
                Message::done("Removed wallpaper from collection.");
            } else {
                Message::fail("Please specify a name of the wallpaper to remove.");
            }
        }
        _ => println!("No subcommand was used"),
    }
    Ok(())
}
