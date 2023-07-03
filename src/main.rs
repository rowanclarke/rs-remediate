#![feature(extend_one, binary_heap_as_slice)]
mod archive;
mod document;
mod file;
mod schedule;
mod session;

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
    Session(SessionAction),
}

#[derive(clap::Args)]
struct SerializeAction {
    path: String,
}

#[derive(clap::Args)]
struct SessionAction {
    #[command(subcommand)]
    command: SessionCommand,
}

#[derive(Subcommand)]
enum SessionCommand {
    Initialize,
    Learn,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Serialize(SerializeAction { path }) => serialize(Path::new(&path)).unwrap(),
        Command::Session(SessionAction {
            command: SessionCommand::Initialize,
        }) => Session::<Data>::new().save(),
        Command::Session(SessionAction {
            command: SessionCommand::Learn,
        }) => Session::<Data>::load().learn(),
    }
}
