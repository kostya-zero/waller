use clap::{Arg, ArgAction, Command};

pub fn app() -> Command {
    Command::new("waller")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommands([
            Command::new("apply")
                .long_about("Applies wallpaper that you have added to collection.")
                .args([
                    Arg::new("name")
                        .help("Name of wallpaper to set.")
                        .required(false)
                        .num_args(1)
                        .value_parser(clap::value_parser!(String)),
                    Arg::new("index")
                        .long("index")
                        .short('i')
                        .help("Index of wallpaper to set.")
                        .required(false)
                        .num_args(1)
                        .value_parser(clap::value_parser!(usize)),
                ]),
            Command::new("add")
                .about("Add image to your collection.")
                .args([
                    Arg::new("path")
                        .required(false)
                        .help("Path to image that you want to add.")
                        .num_args(1)
                        .value_parser(clap::value_parser!(String)),
                    Arg::new("name")
                        .help("Name for new wallpaper (optional).")
                        .num_args(1)
                        .required(false)
                        .value_parser(clap::value_parser!(String))
                        .default_value(""),
                ]),
            Command::new("list")
                .about("List of wallpapers in your collection.")
                .arg(
                    Arg::new("show-indexes")
                        .help("Show indexes for each wallpaper.")
                        .long("show-indexes")
                        .action(ArgAction::SetTrue),
                ),
            Command::new("remove")
                .long_about("Deletes wallpaper from collection by given index.")
                .arg(
                    Arg::new("name")
                        .help("Name of wallpaper in collection.")
                        .required(false)
                        .num_args(1)
                        .value_parser(clap::value_parser!(String)),
                ),
            Command::new("recent").about("Use recent used wall."),
        ])
}
