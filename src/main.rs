mod cli;
mod compression_scheme;
mod nbt;
mod optimizer;
mod region_loader;

use crate::cli::Cli;
use crate::optimizer::optimize_region_file;
use clap::Parser;
use indicatif::ProgressBar;
use rayon::prelude::*;

fn main() {
    let cli = Cli::parse();
    let dir = std::fs::read_dir(cli.path).unwrap();

    let entries = dir.flatten().collect::<Vec<_>>();
    let len = entries.len();
    let pb = ProgressBar::new(len as u64);
    entries.par_iter().for_each(|entry| {
        pb.inc(1);
        let _ = optimize_region_file(entry.path(), &pb);
    });
}
