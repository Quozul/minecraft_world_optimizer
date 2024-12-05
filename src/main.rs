mod cli;
mod commands;
mod nbt;
mod region_loader;
mod world;

use crate::cli::{Cli, Mode};
use crate::commands::read::execute_read;
use crate::commands::write::execute_write;
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    let result = match cli.mode {
        Mode::Write => execute_write(&cli.world_paths),
        Mode::Check => execute_read(&cli.world_paths),
    };

    if let Err(err) = result {
        eprintln!("{err}");
    }
}
