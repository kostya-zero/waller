use args::app;
use config::{ApplyMethod, ApplyMode, Config, Manager};
use proc::Proc;
use std::{path::Path, process::exit};

use crate::term::Term;

mod args;
mod config;
mod proc;
mod term;

fn apply_resolve(method: Option<ApplyMethod>, path: &str, mode: Option<ApplyMode>) {
    if let Some(some_method) = method {
        match some_method {
            ApplyMethod::feh => {
                if let Some(some_mode) = mode {
                    Proc::apply_feh(path, some_mode)
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
    }
}

fn main() {
    if !Manager::is_exists() {
        Manager::make_default_config();
    }

    let mut conf: Config = Manager::get_config();
    let app = app().get_matches();

    match app.subcommand() {
        Some(("set", submatches)) => {
            let path: &str = submatches.get_one::<String>("path").unwrap();

            if !Path::new(&path).exists() {
                Term::fatal("Specified file are not exists in filesystem. Maybe typo error?");
                exit(1);
            }

            apply_resolve(conf.method.clone(), path, conf.mode.clone());
            conf.recent = Some(String::from(path));
            Manager::write_config(conf);
        }
        Some(("apply", _submatches)) => {
            let num = _submatches.get_one::<usize>("index").unwrap_or_else(|| {
                Term::fatal("You have passed wrong argument.");
                exit(1);
            });
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

            apply_resolve(conf.method.clone(), wall, conf.mode.clone());
            conf.recent = Some(wall.to_string());
            Manager::write_config(conf);
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
            Manager::write_config(conf);
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
            Manager::write_config(conf);
            Term::info("Wallpaper remove.");
        }
        Some(("recent", _submatches)) => {
            let recent_wall: &str = &conf.recent.expect("Recent file not specified!");

            if recent_wall.is_empty() {
                Term::fatal("You havent applied any image!");
                exit(1);
            }

            if !Path::new(&recent_wall).exists() {
                Term::fatal("Recent image not found!");
                exit(1);
            }
            apply_resolve(conf.method.clone(), recent_wall, conf.mode.clone());
        }
        _ => Term::fatal("Unknown command! Use \"--help\" option to get help message."),
    }
}
