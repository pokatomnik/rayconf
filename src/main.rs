use crate::cmd::cli::Cli;
use crate::cmd::commands::Commands;
use clap::Parser;

mod cmd;
mod entities;
mod services;
mod utils;
mod v2parser;

fn main() {
    let cli = Cli::parse();

    let error = match cli.command {
        Commands::Add(add_params) => add_params.add(),
        Commands::Delete(delete_params) => delete_params.remove(),
        Commands::Select(select_params) => select_params.select(),
        Commands::Remote(remote_params) => remote_params.handle_action(),
    };

    if let Err(error) = error {
        eprintln!("{}", error);
    }
}
