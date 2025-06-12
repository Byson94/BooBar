mod args;
mod config;

use args::{Cli, Commands};
use clap::Parser;
use config::toml_loader::load_toml_config;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Validate) => {
            match load_toml_config(&cli.config) {
                Ok(cfg) => println!("Valid config:\n{:#?}", cfg),
                Err(e) => eprintln!("Config validation failed: {}", e),
            }
        }
        Some(Commands::Run) | None => {
            match load_toml_config(&cli.config) {
                Ok(cfg) => {
                    println!("Loaded config: {:#?}", cfg);
                    // TODO: runtime logic here
                }
                Err(e) => eprintln!("Error loading config: {}", e),
            }
        }
    }
}
