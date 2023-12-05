use args::app;
use collection::Wallpaper;
use config::{ApplyMethod, ApplyMode};
use manager::{Manager, ManagerError};
use proc::Proc;
use std::{path::Path, process::exit};

use crate::term::Term;

mod args;
mod collection;
mod config;
mod manager;
mod proc;
mod term;
mod utils;

fn apply_resolve(method: Option<ApplyMethod>, path: &str, mode: Option<ApplyMode>) {
    if let Some(some_method) = method {
        match some_method {
            ApplyMethod::feh => {
                if let Some(some_mode) = mode {
                    Proc::apply_feh(path, some_mode)
                } else {
                    Term::fatal("Apply mode not set or not correct. Check your configuration and fill missing field 'mode'.");
                    exit(1);
                }
            }
            ApplyMethod::swaybg => {
                if let Some(some_mode) = mode {
                    Proc::apply_swaybg(path, some_mode)
                }
            }
            ApplyMethod::gnome => Proc::apply_gnome(path),
            ApplyMethod::kde => Proc::apply_kde(path),
        }
    } else {
        Term::fatal("Apply method not set or not correct. Check your configuration and fill missing field 'method'.");
        exit(1);
    }
}

fn main() {
    if !Manager::is_exists() {
        Manager::make_default_config();
    }

    let app = app().get_matches();
    match app.subcommand() {
        Some(("apply", _submatches)) => {
            let init_collection = Manager::load_collection();

            if let Ok(collection) = init_collection {
                if collection.is_empty() {
                    Term::fatal(
                        "Your collection is empty... and dusty. How about to add new wallpaper?",
                    );
                    exit(1);
                }

                let name = _submatches.get_one::<String>("name").unwrap();

                #[allow(unused_assignments)]
                let mut wallpaper: Option<Wallpaper> = None;

                if let Some(num) = _submatches.get_one::<usize>("index") {
                    if let Ok(wall) = collection.get_by_index(*num) {
                        wallpaper = Some(wall);
                    } else {
                        Term::fatal("Wallpaper not found.");
                        exit(1);
                    }
                } else if let Ok(wall) = collection.get_by_name(name) {
                    wallpaper = Some(wall);
                } else {
                    Term::fatal("Wallpaper not found.");
                    exit(1);
                }

                if let Some(wall) = wallpaper {
                    if !Path::new(&wall.get_path()).exists() {
                        Term::fatal("Image file by path doesn't exists! Remove it from list.");
                        exit(1);
                    }

                    Term::info(format!("Applying wallpaper - {}", wall.get_name()).as_str());
                    if let Ok(mut config) = Manager::load_config() {
                        apply_resolve(config.get_method(), wall.get_path(), config.get_mode());
                        config.set_recent(wall);

                        match Manager::write_config(config) {
                            Ok(_) => Term::info("Wallpaper added!"),
                            Err(e) => match e {
                                ManagerError::LoadError(m) => {
                                    Term::fatal(
                                        format!("Failed to write configuration: {}", m).as_str(),
                                    );
                                    exit(1);
                                }
                                _ => {
                                    Term::fatal("Unexpected error.");
                                    exit(1);
                                }
                            },
                        }
                    }
                }
            }
        }
        Some(("add", sub)) => {
            if let Some(image_path) = sub.get_one::<String>("path") {
                let path = Path::new(&image_path);
                if !path.exists() {
                    Term::fatal("File by given path not found!");
                    exit(1);
                }

                let mut name: String = sub.get_one::<String>("name").unwrap().to_string();
                if name.is_empty() {
                    name.push_str(path.file_stem().unwrap().to_str().unwrap());
                }

                let mut new_wallpaper = Wallpaper::new();
                new_wallpaper.set_name(name.as_str());
                new_wallpaper.set_path(image_path);

                match Manager::load_collection() {
                    Ok(mut collection) => {
                        collection.add(new_wallpaper);
                        Manager::write_collection(collection);
                        Term::info("Wallpaper added.")
                    }
                    Err(e) => match e {
                        ManagerError::LoadError(m) => {
                            Term::fatal(format!("Failed to load collection: {m}").as_str());
                            exit(1);
                        }
                        _ => {
                            Term::fatal("Unexpected error occured.");
                            exit(1);
                        }
                    },
                }
            }
        }
        Some(("list", sub)) => {
            if let Ok(collection) = Manager::load_collection() {
                if collection.is_empty() {
                    Term::fatal(
                        "Your collection is empty... and dusty. How about to add new wallpaper?",
                    );
                    exit(1);
                }

                let show_indexes: bool = sub.get_flag("show-indexes");
                for (index, item) in collection.get_collection().iter().enumerate() {
                    if show_indexes {
                        println!("{}. {}", index, item.get_name());
                    } else {
                        println!("{}", item.get_name());
                    }
                }
            }
        }
        Some(("remome", sub)) => {
            let name: &str = sub.get_one::<String>("name").unwrap();
            if let Ok(mut collection) = Manager::load_collection() {
                let mut wall_index: Option<usize> = None;
                if let Some(index) = sub.get_one::<usize>("index") {
                    wall_index = Some(*index);
                } else if !name.is_empty() {
                    if !collection.is_exists(name) {
                        Term::fatal("Wallpaper not found.");
                        exit(1);
                    }

                    if let Ok(index) = collection.get_index(name) {
                        wall_index = Some(index);
                    }
                }

                #[allow(clippy::redundant_pattern_matching)]
                if let Err(_) = collection.remove_by_index(wall_index.unwrap()) {
                    Term::fatal("Wallpaper not found.");
                } else {
                    match Manager::write_collection(collection) {
                        Ok(_) => Term::info("Wallpaper removed."),
                        Err(e) => match e {
                            ManagerError::WriteError(m) => {
                                Term::info(format!("Failed to write collection: {}", m).as_str());
                                exit(1);
                            }
                            _ => {
                                Term::fatal("Unexpected error.");
                                exit(1);
                            }
                        },
                    }
                    Term::info("Wallpaper removed");
                }
            }
        }
        Some(("recent", _submatches)) => {
            if let Ok(config) = Manager::load_config() {
                if let Some(wallpaper) = config.get_recent() {
                    Term::info(format!("Applying wallpaper - {}", wallpaper.get_name()).as_str());
                    apply_resolve(config.get_method(), wallpaper.get_path(), config.get_mode());
                }
            }
        }
        _ => Term::fatal("Unknown command! Use \"--help\" option to get help message."),
    }
}
