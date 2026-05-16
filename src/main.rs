use std::process::ExitCode;

use crate::cmd::cli::Cli;
use crate::cmd::commands::Commands;
use clap::Parser;

mod cmd;
mod entities;
mod services;
mod utils;
mod v2parser;

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();

    let command = async move {
        match cli.command {
            Commands::Add(add_params) => add_params.add().await,
            Commands::Delete(delete_params) => delete_params.remove().await,
            Commands::Select(select_params) => select_params.select().await,
            Commands::Remote(remote_params) => remote_params.handle_action().await,
            Commands::Perf(perf_params) => perf_params.start().await,
        }
    };

    wait_for_exit(command).await
}

async fn wait_for_exit<T, E: std::fmt::Display>(
    fut: impl Future<Output = Result<T, E>>,
) -> ExitCode {
    tokio::select! {
        result = fut => {
            match result {
                Ok(_) => {
                    ExitCode::SUCCESS
                },
                Err(error) => {
                    eprintln!("Rayconf finished with error: {error}");
                    ExitCode::FAILURE
                }
            }
        }

        signal = tokio::signal::ctrl_c() => {
            match signal {
                Ok(()) => {
                    eprintln!("Graceful shutdown");
                    ExitCode::from(130)
                }

                Err(error) => {
                    eprintln!("failed to listen for Ctrl+C: {error}");
                    ExitCode::FAILURE
                }
            }
        }
    }
}
