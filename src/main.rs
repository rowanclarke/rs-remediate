mod learn;
mod serialize;

use learn::learn;
use serialize::serialize;
use std::{collections::BTreeMap, path::Path};

use clap::{Parser, Subcommand};

type Deck<T> = BTreeMap<(T, T), Vec<T>>;
type Answers<T> = BTreeMap<T, Vec<T>>;

const REMEDY_DIR: &str = "REMEDY_DIR";

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Serialize(SerializeAction),
    Learn,
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
        Command::Learn => learn().map_err(|()| "".to_string()),
    }
}
