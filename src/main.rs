mod cli;
mod nbt;
mod optimizer;
mod region_loader;
mod world;

use crate::cli::Cli;
use crate::optimizer::{optimize_region_file, OptimizeResult};
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
            let entries = cli
                .world_path
                .iter()
                .flat_map(|world| get_region_files(world))
                .collect::<Vec<_>>();

            let pb = ProgressBar::new(entries.len() as u64);
            let style = ProgressStyle::with_template(
                "{percent}% {bar} {pos}/{len} [{elapsed_precise}>{eta_precise}, {per_sec}]",
            )
            .unwrap();
            pb.set_style(style);

            let mut results = entries
                .par_iter()
                .map(|entry| {
                    let result = optimize_region_file(entry, cli.write);
                    pb.inc(1);
                    result
                })
                .flatten()
                .collect::<Vec<OptimizeResult>>();

            let result = results.iter_mut().reduce(|acc, cur| {
                acc.deleted_regions += cur.deleted_regions;
                acc.total_chunks += cur.total_chunks;
                acc.deleted_chunks += cur.deleted_chunks;
                acc
            });

            println!("{:?}", result);
        }
    }
}
