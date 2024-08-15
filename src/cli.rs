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
    /// Path to the "region" directory of you Minecraft World
    pub path: PathBuf,
}
