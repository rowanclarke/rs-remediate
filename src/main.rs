#![feature(extend_one, binary_heap_as_slice)]
mod document;
mod file;
mod schedule;
mod session;
mod with;

use document::serialize;
use schedule::sm2::Data;

use clap::{Parser, Subcommand};
use session::Session;
use std::path::Path;

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
        _ => {
            let mut session = Session::<Data>::new();
            loop {
                session.learn();
            }
        }
    }
}
