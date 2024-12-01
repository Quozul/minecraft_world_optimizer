mod cli;
mod nbt;
mod optimizer;
mod region_loader;
mod world;

use crate::cli::Cli;
use crate::optimizer::optimize_region_file;
use crate::world::get_region_files::get_region_files;
use crate::world::validate::validate_worlds;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;

fn main() {
    let cli = Cli::parse();

    match validate_worlds(&cli.world_path) {
        Err(err) => {
            eprintln!("{err}");
        }
        Ok(_) => {
            for world in cli.world_path {
                let entries = get_region_files(&world);

                let pb = ProgressBar::new(entries.len() as u64);
                let style = ProgressStyle::with_template(
                    "{percent}% {bar} {pos}/{len} [{elapsed_precise}>{eta_precise}, {per_sec}]",
                )
                .unwrap();
                pb.set_style(style);

                entries.par_iter().for_each(|entry| {
                    let _ = optimize_region_file(entry, &pb);
                    pb.inc(1);
                });
            }
        }
    }
}
