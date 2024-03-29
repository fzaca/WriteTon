use crate::cli::{Cli, Commands};
use clap::Parser;

mod cli;
mod utils;

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::New { content } => println!("{content}"),
    }
}
