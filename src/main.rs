mod cli;
mod nbt;
mod optimizer;
mod region_loader;

use crate::cli::Cli;
use crate::optimizer::optimize_region_file;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;

fn main() {
    let cli = Cli::parse();

    let entries = std::fs::read_dir(cli.path)
        .map(|dir| dir.flatten().collect::<Vec<_>>())
        .unwrap_or_default();

    let pb = ProgressBar::new(entries.len() as u64);
    let style = ProgressStyle::with_template(
        "{percent}% {bar} {pos}/{len} [{elapsed_precise}>{eta_precise}, {per_sec}]",
    )
    .unwrap();
    pb.set_style(style);

    entries.par_iter().for_each(|entry| {
        let _ = optimize_region_file(entry.path(), &pb);
        pb.inc(1);
    });
}
