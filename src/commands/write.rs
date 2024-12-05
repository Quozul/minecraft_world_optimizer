use crate::commands::optimize_result::{reduce_optimize_results, OptimizeResult};
use crate::region_loader::region::Region;
use crate::world::get_region_files::get_region_files;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;
use std::error::Error;
use std::path::PathBuf;

pub fn execute_write(world_paths: &Vec<PathBuf>) -> Result<(), Box<dyn Error>> {
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
            let result = optimize_write(entry);
            pb.inc(1);
            result
        })
        .flatten()
        .collect::<Vec<OptimizeResult>>();

    let result = reduce_optimize_results(&mut results);
    println!("{result}");

    Ok(())
}

fn optimize_write(region_file_path: &PathBuf) -> std::io::Result<OptimizeResult> {
    let mut result = OptimizeResult::default();

    match Region::from_file_name(region_file_path) {
        Ok(mut region) => {
            let chunks = region.get_chunks();
            result.total_chunks += chunks.len();

            let mut chunks_to_delete = Vec::new();
            for chunk in chunks {
                if chunk.should_delete() {
                    chunks_to_delete.push(chunk.clone());
                }
            }
            result.deleted_chunks += chunks_to_delete.len();

            for chunk in &chunks_to_delete {
                region.remove_chunk(chunk);
            }

            if region.is_empty() {
                result.deleted_regions += 1;
                std::fs::remove_file(region_file_path)?;
            } else {
                let bytes = region.to_bytes();
                std::fs::write(region_file_path, bytes)?;
            }
        }
        Err(_) => {
            result.deleted_regions += 1;
            std::fs::remove_file(region_file_path)?;
        }
    }

    Ok(result)
}
