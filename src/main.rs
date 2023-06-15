mod learn;
mod serialize;

use learn::learn;
use serialize::serialize;
use std::{collections::BTreeMap, path::Path};

use clap::{Parser, Subcommand};

type DeckBorrow<'a> = BTreeMap<(&'a str, &'a str), Vec<&'a str>>;
type DeckOwned = BTreeMap<(String, String), Vec<String>>;

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
