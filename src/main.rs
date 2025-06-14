mod args;
mod config;
mod core;

use args::{Cli, Commands};
use clap::Parser;
use config::rhai_loader::load_rhai_config;
// use config::runtime::Runtime;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Validate) => {
            match load_rhai_config(&cli.config) {
                Ok((cfg, _script)) => println!("Valid config:\n{:#?}", cfg),
                Err(e) => eprintln!("Config validation failed: {}", e),
            }
        },
        Some(Commands::Run { window_name }) => {
            match load_rhai_config(&cli.config) {
                Ok((cfg, _script)) => {
                    core::start_ui(/* rt = */ (), &cfg, window_name);
                }
                Err(e) => eprintln!("Error loading config: {}", e),
            }
        },
        None => {
            eprintln!("No command found: {{cli.command = {:?}}}", cli.command)
        }
    }
}
