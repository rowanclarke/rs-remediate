mod serialize;

use serialize::serialize;
use std::{fs, io, path::Path};

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Serialize(SerializeAction),
}

#[derive(clap::Args)]
struct SerializeAction {
    path: String,
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    match args.command {
        Command::Serialize(SerializeAction { path }) => {
            serialize(Path::new(&path)).map_err(|e| format!("{}", e))
        }
    }
}
