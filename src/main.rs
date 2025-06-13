mod args;
mod config;
mod core;
mod runtime;

use args::{Cli, Commands};
use clap::Parser;
use config::lua_loader::load_lua_config;
use runtime::runtime::Runtime;

fn main() {
    let cli = Cli::parse();

    if let Some(Commands::Validate) = &cli.command {
        match Runtime::from_script(&cli.config) {
            Ok((_rt, cfg)) => println!("Valid config:\n{:#?}", cfg),
            Err(e) => eprintln!("Config validation failed: {}", e),
        }
    } else {
        match Runtime::from_script(&cli.config) {
            Ok((rt, cfg)) => {
                // println!("{:?}", &cfg); // debug line
                core::start_ui(rt, &cfg);
            }
            Err(e) => eprintln!("Error loading config: {}", e),
        }
    }
}
