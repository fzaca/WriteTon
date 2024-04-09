use clap::Parser;

mod cli;
mod commands;
mod database;
mod models;
mod utils;

use crate::cli::{Cli, Commands};

fn main() {
    if let Err(err) = database::ensure_database_created() {
        eprintln!("Error creating the database: {err}");
        std::process::exit(1);
    }

    let args = Cli::parse();

    match args.command {
        Commands::New { content } => commands::new::exec(&content),
        Commands::List => commands::list::exec(),
        Commands::Rm { note_id } => commands::remove::exec(&note_id),
    }
}
