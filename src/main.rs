use std::{process::exit, ffi::OsStr, path::Path};
use clap::{Command, Arg};
use config::{ConfigStruct, ConfigManager};
use proc::Proc;
use walkdir::WalkDir;
use rand::Rng;

use crate::term::Term;

mod config;
mod paths;
mod proc;
mod term;

fn cli() -> Command {
    Command::new("waller")
        .about("Safe wallpaper manager for your desktop.")
        .author(".ZERO")
        .version("0.2.0")
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
            Command::new("random")
                .about("Applies random image from specified directory in random_folder option."),
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
                     .value_parser(clap::value_parser!(usize)))
        ])
}

fn main() {
    if !ConfigManager::is_exists() {
        ConfigManager::make_default_config();
    }
    
    let conf: ConfigStruct = ConfigManager::get_config();
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

            match conf.method {
                config::ApplyMethod::swaybg => Proc::apply_swaybg(path, conf.mode),
                config::ApplyMethod::feh => Proc::apply_feh(path, conf.mode)
            }
        },
        Some(("apply", _submatches)) => {
            let walls = ConfigManager::get_walls();
            let num = _submatches.get_one::<usize>("index").expect("Failed to get index.");

            if num + 1 > walls.len() {
                Term::fatal("Index out of range.".to_string());
                exit(1);
            }

            let wall = &walls[*num];
            
            if !Path::new(&wall).exists() {
                Term::fatal("Image file by path doesn't exists! Remove it from list.".to_string());
                exit(1);
            }

            term::Term::info(format!("Applying image: {}", wall));

            match conf.method {
                config::ApplyMethod::swaybg => Proc::apply_swaybg(wall.to_string(), conf.mode),
                config::ApplyMethod::feh => Proc::apply_feh(wall.to_string(), conf.mode)
            }

        },
        Some(("random", _submatches)) => {
            if conf.random_folder == None {
                Term::fatal("Folder with pictures are not specified in config.toml.".to_string());
                exit(1);
            }
            let path: String = conf.random_folder.expect("Error").trim().to_string();
            
            if path == "" {
                Term::fatal("The `random_folder` option does not specify the directory from where to take the images.".to_string());
                exit(1);
            }

            if !Path::new(&path).exists() {
                Term::fatal("Directory that you specify doesn't exists.".to_string());
                exit(1);
            }

            let mut files: Vec<String> = Vec::new(); 

            for file in WalkDir::new(&path).into_iter().filter_map(|file| file.ok()) {
                    files.push(file.path().display().to_string());
            }
            let mut rng = rand::thread_rng();

            let image_path: &str;
            loop {
                let num = rng.gen_range(1..files.len());
                let picture = &files[num];
                let ext: &str = Path::new(picture).extension().and_then(OsStr::to_str).expect("Fail");
                let supported_ext = vec!["png", "jpg", "jpeg"];

                if supported_ext.iter().any(|&e| e==ext) {
                    image_path = picture;
                    break; 
                }
            }
            match conf.method {
                config::ApplyMethod::swaybg => Proc::apply_swaybg(image_path.to_string(), conf.mode),
                config::ApplyMethod::feh => Proc::apply_feh(image_path.to_string(), conf.mode)
            }
        },
        Some(("add", submatches)) => {
            let path: String = submatches.get_one::<String>("path").expect("Failed to get path.").trim().to_string();

            if !Path::new(&path).exists() {
                Term::fatal("File by given path not found!".to_string());
                exit(1);
            }

            let mut walls: Vec<String> = ConfigManager::get_walls();
            for wall in &walls {
                if wall == &path {
                    Term::fatal("Image with same pat already added.".to_string());
                    exit(1);
                }
            }

            walls.push(path);
            ConfigManager::write_walls(walls);
            Term::info("Image added.".to_string())
        },
        Some(("list", _submatches)) => {
            let walls: Vec<String> = ConfigManager::get_walls();
            let mut num: usize = 0;
            for wall in &walls {
                println!("{} : {}", num.to_string(), wall);
                num += 1;
            }
        },
        Some(("rm", _submatches)) => {
            let mut walls = ConfigManager::get_walls();
            let num = _submatches.get_one::<usize>("index").expect("Failed to get index.");

            if num + 1 > walls.len() {
                Term::fatal("Index out of range.".to_string());
                exit(1);
            }

            walls.remove(*num);
            ConfigManager::write_walls(walls);
            Term::info("Wallpaper remove.".to_string());
        }
        _ => Term::fatal("Unknown command! Use \"--help\" option to get help message.".to_string())
    }
}
