use crate::region_loader::region::Region;
use std::path::PathBuf;

#[derive(Default, Debug, Clone)]
pub struct OptimizeResult {
    pub total_chunks: usize,
    pub deleted_chunks: usize,
    pub deleted_regions: usize,
}

pub fn optimize_region_file(
    region_file_path: &PathBuf,
    write: bool,
) -> std::io::Result<OptimizeResult> {
    let mut result = OptimizeResult::default();

    match Region::from_file_name(region_file_path) {
        Ok(mut region) => {
            let chunks = region.get_chunks();
            result.total_chunks += chunks.len();

            if write {
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
            } else {
                for chunk in chunks {
                    if chunk.should_delete() {
                        result.deleted_chunks += 1;
                    }
                }
                if result.deleted_chunks >= result.total_chunks {
                    result.deleted_regions += 1;
                }
            }
        }
        Err(_) => {
            result.deleted_regions += 1;
            if write {
                std::fs::remove_file(region_file_path)?;
            }
        }
    }

    Ok(result)
}
