use std::process::{Command, Stdio};
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
    
    pub fn apply_kde(path: String) {
        let mut cmd = Command::new("dbus-send");
        cmd.args(vec![
                "--session",
                "--dest=org.kde.plasmashell",
                "--type=method_call",
                "/PlasmaShell",
                "org.kde.PlasmaShell.evaluateScript",
                format!("string: \nvar allDesktops = desktops();\nprint (allDesktops);\nfor (i=0;i<allDesktops.length;i++) {{\nd = allDesktops[i];\nd.wallpaperPlugin = \"org.kde.image\";\nd.currentConfigGroup = Array(\"Wallpaper\", \"org.kde.image\", \"General\");\nd.writeConfig(\"Image\", \"{}\")\n}}", path).as_str()
            ]);
        cmd.stderr(Stdio::inherit()).stdout(Stdio::inherit());
        cmd.output().expect("Failed to run dbus-send.");

        Term::info("Done.");
    }
}
