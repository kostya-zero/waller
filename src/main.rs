use std::{process::exit, ffi::OsStr};
use clap::{Command, Arg};
use config::{ConfigStruct, ConfigManager};
use proc::Proc;
use std::path::Path;
use walkdir::WalkDir;
use rand::Rng;

mod config;
mod paths;
mod proc;

fn cli() -> Command {
    Command::new("waller")
        .about("Safe wallpaper manager for your desktop.")
        .author(".ZERO")
        .version("0.1.0")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(Command::new("set")
                    .about("Set picture as wallpaper.")
                    .arg_required_else_help(true)
                    .arg(Arg::new("path")
                         .required(true)
                         .help("Path to picture that you want to apply.")
                         .num_args(1)
                         .value_parser(clap::value_parser!(String))
                    )
        .subcommand(Command::new("apply")
                    .about("Applies wallpaper that specified in config as default_wall.")
                    )
        .subcommand(Command::new("random")
                    .about("Applies random from specified directory in random_folder option.")
                    )
        )
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
                config::ApplyMethod::feh => Proc::apply_feh(path, conf.mode),
            }
        },
        Some(("apply", _submatches)) => {
            if conf.default_wall.trim() == "" {
                println!("No wallpaper specified to default_wall option.");
                exit(1);
            }

            if !Path::new(&conf.default_wall).exists() {
                println!("Specified file are not exists in filesystem. Maybe typo error?");
                exit(1);
            }
            
            match conf.method {
                config::ApplyMethod::swaybg => Proc::apply_swaybg(conf.default_wall, conf.mode),
                config::ApplyMethod::feh => Proc::apply_feh(conf.default_wall, conf.mode),
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
        }
        _ => println!("Unknown command!")
    }
}
