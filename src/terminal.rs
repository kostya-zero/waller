use std::process::exit;

pub struct Message;

impl Message {
    fn print_message(prefix: &str, color_code: &str, msg: &str) {
        println!("\x1b[{}m{}\x1b[0m {}", color_code, prefix, msg);
    }

    pub fn error(msg: &str) {
        Self::print_message("", "91", msg);
    }

    pub fn done(msg: &str) {
        Self::print_message("", "92", msg);
    }

    pub fn busy(msg: &str) {
        Self::print_message("󰦖", "97", msg);
    }

    pub fn info(msg: &str) {
        Self::print_message("󰍡", "97", msg);
    }

    pub fn list_title(msg: &str) {
        Self::print_message("", "97", msg);
    }

    pub fn item(msg: &str) {
        println!(" {}", msg);
    }

    pub fn fail(msg: &str) {
        Self::error(msg);
        exit(1);
    }
}
