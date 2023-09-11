use clap::{Arg, Command};
use config::{ApplyMethod, ApplyMode, ConfigManager, ConfigStruct};
use proc::Proc;
use std::{path::Path, process::exit};

use crate::term::Term;

mod config;
mod paths;
mod proc;
mod term;

fn cli() -> Command {
    Command::new("waller")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommands([
            Command::new("set")
                .about("Set given path to image as wallpaper.")
                .arg_required_else_help(true)
                .arg(
                    Arg::new("path")
                        .required(true)
                        .help("Path to image that you want to apply.")
                        .num_args(1)
                        .value_parser(clap::value_parser!(String)),
                ),
            Command::new("apply")
                .long_about("Applies wallpaper that you have added to collection.")
                .arg(
                    Arg::new("index")
                        .help("Index of image in collection.")
                        .required(true)
                        .num_args(1)
                        .value_parser(clap::value_parser!(usize)),
                ),
            Command::new("add")
                .about("Add image to your collection.")
                .arg(
                    Arg::new("path")
                        .required(true)
                        .help("Path to image that you want to add.")
                        .num_args(1)
                        .value_parser(clap::value_parser!(String)),
                ),
            Command::new("list").about("List of wallpapers in your collection."),
            Command::new("rm")
                .long_about("Deletes wallpaper from collection by given index.")
                .arg(
                    Arg::new("index")
                        .help("Index of image in collection.")
                        .required(true)
                        .num_args(1)
                        .value_parser(clap::value_parser!(usize)),
                ),
            Command::new("recent").about("Use recent used wall."),
        ])
}

fn apply_resolve(method: ApplyMethod, path: String, mode: ApplyMode) {
    match method {
        ApplyMethod::feh => Proc::apply_feh(path, mode),
        ApplyMethod::swaybg => Proc::apply_swaybg(path, mode),
        ApplyMethod::gnome => Proc::apply_gnome(path),
        ApplyMethod::kde => Proc::apply_kde(path),
    }
}

fn main() {
    if !ConfigManager::is_exists() {
        ConfigManager::make_default_config();
    }

    let mut conf: ConfigStruct = ConfigManager::get_config();
    let app = cli().get_matches();

    // Use it only for debug!!!
    // println!("{:?}", conf);

    match app.subcommand() {
        Some(("set", submatches)) => {
            let path: String = submatches
                .get_one::<String>("path")
                .expect("Failed to get user command line.")
                .to_string();

            if !Path::new(&path).exists() {
                Term::fatal("Specified file are not exists in filesystem. Maybe typo error?");
                exit(1);
            }

            let method = conf.method.clone().expect("Apply method not specified!");
            let mode = conf.mode.clone().expect("Apply mode not specified!");

            apply_resolve(method, path.clone(), mode);
            conf.recent = Some(path);
            ConfigManager::write_config(conf);
        }
        Some(("apply", _submatches)) => {
            let num = _submatches
                .get_one::<usize>("index")
                .expect("Failed to get index.");
            let walls = &conf.walls.clone().expect("Walls are not specified!");

            if num > &walls.len() {
                Term::fatal("Index out of range.");
                exit(1);
            }

            let wall = &walls[*num];

            if !Path::new(&wall).exists() {
                Term::fatal("Image file by path doesn't exists! Remove it from list.");
                exit(1);
            }

            term::Term::info(format!("Applying image: {}", wall).as_str());

            let method = conf.method.clone().expect("Apply method not specified!");
            let mode = conf.mode.clone().expect("Apply mode not specified!");

            apply_resolve(method, wall.to_string(), mode);
            conf.recent = Some(wall.to_string());
            ConfigManager::write_config(conf);
        }
        Some(("add", submatches)) => {
            let path: String = submatches
                .get_one::<String>("path")
                .expect("Failed to get path.")
                .trim()
                .to_string();

            if !Path::new(&path).exists() {
                Term::fatal("File by given path not found!");
                exit(1);
            }

            let mut walls: Vec<String> = conf.walls.expect("Walls are not specified!");
            if walls.iter().any(|p| p == &path) {
                Term::fatal("Image with same path already added.");
                exit(1);
            }

            walls.push(path);
            conf.walls = Some(walls);
            ConfigManager::write_config(conf);
            Term::info("Image added.")
        }
        Some(("list", _submatches)) => {
            let walls: Vec<String> = conf.walls.expect("Walls are not specified!");

            if walls.is_empty() {
                Term::fatal("No walls in collection!");
                exit(1);
            }

            for (num, wall) in walls.iter().enumerate() {
                println!("{} : {}", num, wall);
            }
        }
        Some(("rm", _submatches)) => {
            let mut walls = conf.walls.expect("Walls are not specified!");
            let num = _submatches
                .get_one::<usize>("index")
                .expect("Failed to get index.");

            if num + 1 > walls.len() {
                Term::fatal("Index out of range.");
                exit(1);
            }

            walls.remove(*num);
            conf.walls = Some(walls);
            ConfigManager::write_config(conf);
            Term::info("Wallpaper remove.");
        }
        Some(("recent", _submatches)) => {
            let recent_wall: String = conf.recent.expect("Recent file not specified!");

            if recent_wall.is_empty() {
                Term::fatal("You havent applied any image!");
                exit(1);
            }

            if !Path::new(&recent_wall).exists() {
                Term::fatal("Recent image not found!");
                exit(1);
            }
            let method = conf.method.expect("Apply method not specified!");
            let mode = conf.mode.expect("Apply mode not specified!");
            apply_resolve(method, recent_wall, mode);
        }
        _ => Term::fatal("Unknown command! Use \"--help\" option to get help message."),
    }
}
