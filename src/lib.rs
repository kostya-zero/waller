use anyhow::Result;
use args::build_cli;
use collection::Collection;
use config::Config;
use platform::Platform;

mod args;
mod collection;
mod config;
mod platform;
mod terminal;

pub fn main() -> Result<()> {
    if !Platform::check_config_exists() {
        let config = Config::default();
        config.save()?;
    }

    if !Platform::check_collection_exists() {
        let collection = Collection::new();
        collection.save()?;
    }

    let args = build_cli();

    match args.subcommand() {
        Some(("add", sub)) => {}
        _ => println!("No subcommand was used"),
    }
    Ok(())
}
