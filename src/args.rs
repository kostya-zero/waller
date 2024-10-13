use clap::{value_parser, Arg, ArgMatches, Command};

pub fn build_cli() -> ArgMatches {
    Command::new("waller")
        .name("waller")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .arg_required_else_help(true)
        .subcommands([
            Command::new("add")
                .about("Add wallpaper to collection")
                .args([
                    Arg::new("path")
                        .help("Path to the wallpaper.")
                        .required(false)
                        .num_args(1)
                        .value_parser(value_parser!(String)),
                    Arg::new("name")
                        .help("Set custom name for wallpaper.")
                        .value_parser(value_parser!(String))
                        .short('n')
                        .long("name")
                        .num_args(1),
                ]),
            Command::new("list").about("Get list of wallpapers in collection."),
            Command::new("remove")
                .about("Remove wallpaper from collection")
                .arg(
                    Arg::new("name")
                        .help("Name of the wallpaper to remove from collection.")
                        .value_parser(value_parser!(String))
                        .required(false)
                        .num_args(1),
                ),
        ])
        .get_matches()
}
