use crate::cmd::cli::Cli;
use crate::cmd::commands::Commands;
use clap::Parser;

mod cmd;
mod entities;
mod services;

fn main() {
    let cli = Cli::parse();

    let error = match cli.command {
        Commands::Add(add_params) => add_params.add(),
        Commands::Delete(delete_params) => delete_params.remove(),
        Commands::Select(select_params) => select_params.select(),
    };

    if let Err(error) = error {
        eprintln!("{}", error);
    }
}
