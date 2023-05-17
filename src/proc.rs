use std::process::Command;
use crate::{config::{self, ApplyMode}, term::Term};

pub struct Proc;
impl Proc {
    pub fn kill_process(name: &str) {
        Command::new("killall")
            .arg(name)
            .output()
            .expect("Failed to end process.");
    }

    pub fn apply_swaybg(path: String, mode: config::ApplyMode) {
        let apply_mode: &str = match mode {
            ApplyMode::fit => "fit",
            ApplyMode::center => "center",
            ApplyMode::fill => "fill",
            ApplyMode::stretch => "stretch"
        };
        let proc_args: Vec<&str> = vec!["--image", path.as_str(), "--mode", apply_mode];

        Proc::kill_process("swaybg");

        Command::new("swaybg")
            .args(proc_args)
            .spawn()
            .unwrap();

        Term::info("Done.");
    }

    pub fn apply_feh(path: String, mode: config::ApplyMode) {
        let apply_mode: &str = match mode {
            ApplyMode::fit => "--bg-max",
            ApplyMode::center => "--bg-center",
            ApplyMode::fill => "--bg-fill",
            ApplyMode::stretch => "--bg-scale"
        };
        Proc::kill_process("feh");
    
        let proc_args: Vec<&str> = vec![apply_mode, path.as_str()]; 

        Command::new("feh")
            .args(proc_args)
            .spawn()
            .unwrap();

        Term::info("Done.");
    }

    pub fn apply_gnome(path: String) {
        Command::new("gsettings")
            .args(vec!["set", "org.gnome.desktop.background", "picture-uri", &path])
            .output()
            .expect("Failed to call gsettings!");

        Command::new("gsettings")
            .args(vec!["set", "org.gnome.desktop.background", "picture-uri-dark", &path])
            .output()
            .expect("Failed to call gsettings!");

        Term::info("Done.");
    }

}
