use std::{process::exit, ffi::OsStr, path::Path};
use clap::{Command, Arg};
use config::{ConfigStruct, ConfigManager};
use proc::Proc;
use walkdir::WalkDir;
use rand::Rng;

mod config;
mod paths;
mod proc;

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
    match app.subcommand() {
        Some(("set", submatches)) => {

            let path: String = submatches.get_one::<String>("path").expect("Failed to get user command line.").to_string();

            if !Path::new(&path).exists() {
                println!("Specified file are not exists in filesystem. Maybe typo error?");
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
                println!("Index out of range.");
                exit(1);
            }

            let wall = &walls[*num];
            println!("Applying image: {}", wall);

            match conf.method {
                config::ApplyMethod::swaybg => Proc::apply_swaybg(wall.to_string(), conf.mode),
                config::ApplyMethod::feh => Proc::apply_feh(wall.to_string(), conf.mode)
            }

        },
        Some(("random", _submatches)) => {
            let path: String = conf.random_folder.trim().to_string();
            
            if path == "" {
                println!("The `random_folder` option does not specify the directory from where to take the images.");
                exit(1);
            }

            if !Path::new(&path).exists() {
                println!("Directory that you specify doesn't exists.");
                exit(1);
            }

            let mut files: Vec<String> = Vec::new(); 

            for file in WalkDir::new(&path).into_iter().filter_map(|file| file.ok()) {
                    files.push(file.path().display().to_string());
            }
            let mut rng = rand::thread_rng();

            let mut image_path: &str = "";
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
                println!("File by given path not found!");
                exit(1);
            }

            let mut walls: Vec<String> = ConfigManager::get_walls();
            for wall in &walls {
                if wall == &path {
                    println!("Image with same path already added.");
                    exit(1);
                }
            }

            walls.push(path);
            ConfigManager::write_walls(walls);
            println!("Added.")
        },
        Some(("list", submatches)) => {
            let mut walls: Vec<String> = ConfigManager::get_walls();
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
                println!("Index out of range.");
                exit(1);
            }

            walls.remove(*num);
            ConfigManager::write_walls(walls);
            println!("Wallpaper removed.");
        }
        _ => println!("Unknown command!")
    }
}
