use clap::{Arg, Command};

pub fn app() -> Command {
    Command::new("waller")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommands([
            Command::new("set")
                .about("Set given path to image as wallpaper.")
                .arg_required_else_help(true)
                .arg(
                    Arg::new("path")
                        .required(true)
                        .help("Path to image that you want to apply.")
                        .num_args(1)
                        .value_parser(clap::value_parser!(String)),
                ),
            Command::new("apply")
                .long_about("Applies wallpaper that you have added to collection.")
                .arg(
                    Arg::new("index")
                        .help("Index of image in collection.")
                        .required(true)
                        .num_args(1)
                        .value_parser(clap::value_parser!(usize)),
                ),
            Command::new("add")
                .about("Add image to your collection.")
                .arg(
                    Arg::new("path")
                        .required(true)
                        .help("Path to image that you want to add.")
                        .num_args(1)
                        .value_parser(clap::value_parser!(String)),
                ),
            Command::new("list").about("List of wallpapers in your collection."),
            Command::new("rm")
                .long_about("Deletes wallpaper from collection by given index.")
                .arg(
                    Arg::new("index")
                        .help("Index of image in collection.")
                        .required(true)
                        .num_args(1)
                        .value_parser(clap::value_parser!(usize)),
                ),
            Command::new("recent").about("Use recent used wall."),
        ])
}
