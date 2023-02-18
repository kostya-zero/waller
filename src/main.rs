use clap::{Command, Arg};
use config::{ConfigStruct, ConfigManager};
use proc::Proc;

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
            // println!("{:?}", conf);
            // Proc::apply_swaybg(submatches.get_one::<String>("path").expect("Failed.").to_string(), conf.mode);
            match conf.method {
                config::ApplyMethod::swaybg => Proc::apply_swaybg(submatches.get_one::<String>("path").expect("Failed.").to_string(), conf.mode),
                config::ApplyMethod::feh => Proc::apply_feh(submatches.get_one::<String>("path").expect("Failed.").to_string(), conf.mode),

 
            }
        }
        _ => println!("Unknown command!")
    }
}
