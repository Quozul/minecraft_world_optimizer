use crate::commands::optimize_result::{reduce_optimize_results, OptimizeResult};
use crate::region_loader::region::Region;
use crate::world::get_region_files::get_region_files;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;
use std::error::Error;
use std::path::PathBuf;

pub fn execute_read(world_paths: &Vec<PathBuf>) -> Result<(), Box<dyn Error>> {
    let entries = get_region_files(world_paths)?;
    let pb = ProgressBar::new(entries.len() as u64);
    let style = ProgressStyle::with_template(
        "{percent}% {bar} {pos}/{len} [{elapsed_precise}>{eta_precise}, {per_sec}]",
    )
    .unwrap();
    pb.set_style(style);

    let mut results = entries
        .par_iter()
        .map(|entry| {
            let result = optimize_read(entry);
            pb.inc(1);
            result
        })
        .flatten()
        .collect::<Vec<OptimizeResult>>();

    let result = reduce_optimize_results(&mut results);
    println!("{result}");

    Ok(())
}

fn optimize_read(region_file_path: &PathBuf) -> std::io::Result<OptimizeResult> {
    let mut result = OptimizeResult::default();

    match Region::from_file_name(region_file_path) {
        Ok(region) => {
            let chunks = region.get_chunks();
            result.total_chunks += chunks.len();

            for chunk in chunks {
                if chunk.should_delete() {
                    result.deleted_chunks += 1;
                }
            }
            if result.deleted_chunks >= result.total_chunks {
                result.deleted_regions += 1;
            }
        }
        Err(_) => {
            result.deleted_regions += 1;
        }
    }

    Ok(result)
}
