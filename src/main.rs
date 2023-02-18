use std::process::exit;
use clap::{Command, Arg};
use config::{ConfigStruct, ConfigManager};
use proc::Proc;
use std::path::Path;

mod config;
mod paths;
mod proc;

fn cli() -> Command {
    Command::new("waller")
        .about("Safe application to apply pictures as wallpaper.")
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
        )
}

fn main() {
    if !config::ConfigManager::is_exists() {
        println!("Cannot find configuration file.");
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

        }
        _ => println!("Unknown command!")
    }
}
