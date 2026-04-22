use clap::Parser;
use crate::cmd::commands::Commands;

#[derive(Parser)]
#[command(name = "mockers")]
#[command(about = "Simple mock server written in Rust")]
#[command(version)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub command: Commands
}