use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "writeton")] // TODO: Add about
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)] // TODO: Add input when no flag is passed
pub enum Commands {
    New {
        #[arg(short = 'c', long)] // TODO: Add about
        content: String,
    },
    List,
    Rm {
        // TODO: Add tui selector if not argument
        #[arg(index = 1)] // positional argument
        note_id: String,
    },
}
