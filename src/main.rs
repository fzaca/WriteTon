use crate::cli::{Cli, Commands};
use clap::Parser;

mod cli;
mod commands;
mod database;
mod utils;

fn main() {
    let args = Cli::parse();

    if let Err(err) = database::ensure_database_created() {
        println!("Error creating the database: {err}");
        std::process::exit(1);
    }

    match args.command {
        Commands::New { content } => commands::new::exec(content),
    }
}
