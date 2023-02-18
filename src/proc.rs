use std::process::Command;
use crate::config::{self, ApplyMode};

pub struct Proc;
impl Proc {
   pub fn kill_process(name: &str) {
       Command::new("killall")
           .arg(name)
           .output();
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
   }
}
