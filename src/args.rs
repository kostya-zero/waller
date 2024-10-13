use clap::{ArgMatches, Command};

pub fn build_cli() -> ArgMatches {
    Command::new("waller")
        .name("waller")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .arg_required_else_help(true)
        .subcommands([Command::new("add")])
        .get_matches()
}
