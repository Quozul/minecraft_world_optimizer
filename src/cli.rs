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
    /// Path to your Minecraft Worlds containing `level.dat` file.
    pub world_path: Vec<PathBuf>,

    /// Optimizes the world, if not set, will only count regions and chunks to be deleted.
    #[arg(short, long)]
    pub write: bool,
}
