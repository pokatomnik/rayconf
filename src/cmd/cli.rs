use crate::cmd::commands::Commands;
use clap::Parser;

#[derive(Parser)]
#[command(name = "rayconf")]
#[command(about = "XRay configuration utility")]
#[command(version)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
