use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "minecraft_world_optimizer",
    version = "1.0",
    about = "Optimizing Minecraft region files by deleting unused chunks.",
    long_about = None,
)]
pub struct Cli {
    /// Path to your Minecraft World containing `level.dat` file.
    pub world_path: Vec<PathBuf>,
}
