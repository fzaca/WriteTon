use crate::cli::{Cli, Commands};
use clap::Parser;

mod cli;
mod commands;
mod database;
mod utils;

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::New { content } => commands::new::exec(content),
    }
}
