use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "boobar", version, about = "A bar that says BOOO on your config")]
pub struct Cli {
    #[arg(short, long, default_value = "/usr/share/boobar/examples/main/config.toml")]
    pub config: String,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Validate,
    Run,
}
