use crate::{
    config::{self, ApplyMode},
    term::Term,
};
use std::process::{exit, Command, Stdio};

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
            ApplyMode::stretch => "stretch",
        };
        let proc_args: Vec<&str> = vec!["--image", path.as_str(), "--mode", apply_mode];

        Proc::kill_process("swaybg");

        let mut cmd = Command::new("swaybg");
        cmd.args(proc_args);
        let result = cmd.spawn();

        if result.is_err() {
            Term::fatal("Failed to launch 'swaybg'.");
            exit(1);
        }

        Term::info("Done.");
    }

    pub fn apply_feh(path: String, mode: config::ApplyMode) {
        let apply_mode: &str = match mode {
            ApplyMode::fit => "--bg-max",
            ApplyMode::center => "--bg-center",
            ApplyMode::fill => "--bg-fill",
            ApplyMode::stretch => "--bg-scale",
        };
        Proc::kill_process("feh");

        let proc_args: Vec<&str> = vec![apply_mode, path.as_str()];

        let mut cmd = Command::new("feh");
        cmd.args(proc_args);
        let result = cmd.spawn();

        if result.is_err() {
            Term::fatal("Failed to launch 'feh'.");
            exit(1);
        }

        Term::info("Done.");
    }

    pub fn apply_gnome(path: String) {
        let mut cmd = Command::new("gsettings");
        cmd.args(vec![
            "set",
            "org.gnome.desktop.background",
            "picture-uri",
            &path,
        ]);
        let result = cmd.output();

        if result.is_err() {
            Term::fatal("Failed to launch 'gsettings'.");
            exit(1);
        }

        let mut cmd = Command::new("gsettings");
        cmd.args(vec![
            "set",
            "org.gnome.desktop.background",
            "picture-uri-dark",
            &path,
        ]);
        let result2 = cmd.output();

        if result2.is_err() {
            Term::fatal("Failed to launch 'gsettings'.");
            exit(1);
        }

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
        let result = cmd.output();
        if result.is_err() {
            Term::fatal("Failed to launch 'dbus-send'.");
            exit(1);
        }
        Term::info("Done.");
    }
}
