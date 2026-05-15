use crate::cmd::cli::Cli;
use crate::cmd::commands::Commands;
use clap::Parser;

mod cmd;
mod entities;
mod services;
mod utils;
mod v2parser;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let error = match cli.command {
        Commands::Add(add_params) => add_params.add().await,
        Commands::Delete(delete_params) => delete_params.remove().await,
        Commands::Select(select_params) => select_params.select().await,
        Commands::Remote(remote_params) => remote_params.handle_action().await,
        Commands::Perf(perf_params) => perf_params.start().await,
    };

    if let Err(error) = error {
        eprintln!("{}", error);
    }
}
