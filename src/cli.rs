use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "minecraft_world_optimizer",
    version = "1.0",
    about = "Optimizing Minecraft worlds by deleting unused region files and chunks.",
    long_about = None,
)]
pub struct Cli {
    /// What mode to run the program in
    #[arg(value_enum, required = true)]
    pub mode: Mode,

    /// Path to your Minecraft Worlds containing `level.dat` file
    #[arg(required = true)]
    pub world_paths: Vec<PathBuf>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Mode {
    /// Only counts of region files and chunks that can be deleted without making any change to the world
    Check,

    /// Optimizes the world by deleting unused region files and chunks.
    /// This is a destructive process, make sure to make a backup of your worlds before running.
    /// Also make sure the world is not loaded by the game as this will corrupt the world.
    Write,
}
