use std::{process::exit, path::Path};
use clap::{Command, Arg};
use config::{ConfigStruct, ConfigManager, ApplyMode, ApplyMethod};
use proc::Proc;

use crate::term::Term;

mod config;
mod paths;
mod proc;
mod term;

fn cli() -> Command {
    Command::new("waller")
        .about("Safe wallpaper manager for your desktop.")
        .author(".ZERO")
        .version("0.3.0")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommands([ 
            Command::new("set")
                .about("Set given path to image as wallpaper.")
                .arg_required_else_help(true)
                .arg(Arg::new("path")
                    .required(true)
                    .help("Path to image that you want to apply.")
                    .num_args(1)
                    .value_parser(clap::value_parser!(String))),

            Command::new("apply")
                .long_about("Applies wallpaper that you have added to collection.")
                .arg(Arg::new("index")
                     .help("Index of image in collection.")
                     .required(true)
                     .num_args(1)
                     .value_parser(clap::value_parser!(usize))),

            Command::new("add")
                .about("Add image to your collection.")
                .arg(Arg::new("path")
                    .required(true)
                    .help("Path to image that you want to add.")
                    .num_args(1)
                    .value_parser(clap::value_parser!(String))),

            Command::new("list")
                .about("List of wallpapers in your collection."),

            Command::new("rm")
                .long_about("Deletes wallpaper from collection by given index.")
                .arg(Arg::new("index")
                     .help("Index of image in collection.")
                     .required(true)
                     .num_args(1)
                     .value_parser(clap::value_parser!(usize))),

            Command::new("recent")
                .about("Use recent used wall.")
        ])
}

fn apply_resolve(method: ApplyMethod, path: String, mode: ApplyMode) {
    match method {
        ApplyMethod::feh => Proc::apply_feh(path, mode),
        ApplyMethod::swaybg => Proc::apply_swaybg(path,mode),
        ApplyMethod::gnome => Proc::apply_gnome(path)
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

            let path: String = submatches.get_one::<String>("path").expect("Failed to get user command line.").to_string();

            if !Path::new(&path).exists() {
                Term::fatal("Specified file are not exists in filesystem. Maybe typo error?".to_string());
                exit(1);
            }
            apply_resolve(conf.method.clone(), path.clone(), conf.mode.clone());
            conf.recent = path;
            ConfigManager::write_config(conf);
        },
        Some(("apply", _submatches)) => {
            let num = _submatches.get_one::<usize>("index").expect("Failed to get index.");
            let walls = &conf.walls;

            if num > &walls.len() {
                Term::fatal("Index out of range.".to_string());
                exit(1);
            }

            let wall = &walls[*num];
            
            if !Path::new(&wall).exists() {
                Term::fatal("Image file by path doesn't exists! Remove it from list.".to_string());
                exit(1);
            }

            term::Term::info(format!("Applying image: {}", wall));
            apply_resolve(conf.method.clone(), wall.to_string(), conf.mode.clone());
            conf.recent = wall.to_string();
            ConfigManager::write_config(conf);
        },
        Some(("add", submatches)) => {
            let path: String = submatches.get_one::<String>("path").expect("Failed to get path.").trim().to_string();

            if !Path::new(&path).exists() {
                Term::fatal("File by given path not found!".to_string());
                exit(1);
            }

            let mut walls: Vec<String> = conf.walls;
            for wall in &walls {
                if wall == &path {
                    Term::fatal("Image with same path already added.".to_string());
                    exit(1);
                }
            }

            walls.push(path);
            conf.walls = walls;
            ConfigManager::write_config(conf);
            Term::info("Image added.".to_string())
        },
        Some(("list", _submatches)) => {
            let walls: Vec<String> = conf.walls;
            
            if walls.len() == 0 {
                Term::fatal("No walls in collection!".to_string());
                exit(1);
            }

            let mut num: usize = 0;
            for wall in &walls {
                println!("{} : {}", num.to_string(), wall);
                num += 1;
            }
        },
        Some(("rm", _submatches)) => {
            let mut walls = conf.walls;
            let num = _submatches.get_one::<usize>("index").expect("Failed to get index.");

            if num + 1 > walls.len() {
                Term::fatal("Index out of range.".to_string());
                exit(1);
            }

            walls.remove(*num);
            conf.walls = walls;
            ConfigManager::write_config(conf);
            Term::info("Wallpaper remove.".to_string());
        },
        Some(("recent", _submatches)) => {
            if conf.recent == "" {
                Term::fatal("You havent applied any image!".to_string());
                exit(1);
            }

            if !Path::new(&conf.recent).exists() {
                Term::fatal("Recent image not found!".to_string());
                exit(1);
            }

            apply_resolve(conf.method, conf.recent, conf.mode);
        }
        _ => Term::fatal("Unknown command! Use \"--help\" option to get help message.".to_string())
    }
}
