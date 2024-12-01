use crate::region_loader::region::Region;
use indicatif::ProgressBar;
use std::path::PathBuf;

pub fn optimize_region_file(region_file_path: &PathBuf, pb: &ProgressBar) -> std::io::Result<()> {
    let region_file_name = region_file_path
        .file_stem()
        .map(|os_str_val| os_str_val.to_string_lossy().into_owned())
        .unwrap_or("Unk".to_string());

    match Region::from_file_name(region_file_path) {
        Ok(mut region) => {
            // Read the first chunk
            let mut chunks_to_delete = Vec::new();
            for chunk in region.get_chunks() {
                if chunk.should_delete() {
                    chunks_to_delete.push(chunk.clone());
                }
            }
            for chunk in &chunks_to_delete {
                region.remove_chunk(chunk);
            }

            if region.is_empty() {
                pb.println(format!("[{:<15}] Deleted empty region", region_file_name));
                std::fs::remove_file(region_file_path)?;
                return Ok(());
            }

            let deleted_chunk_count = chunks_to_delete.len();
            if deleted_chunk_count > 0 {
                pb.println(format!(
                    "[{:<15}] {} chunks deleted",
                    region_file_name,
                    chunks_to_delete.len()
                ));
            }

            let bytes = region.to_bytes();
            std::fs::write(region_file_path, bytes)?;
        }
        Err(err) => {
            pb.println(format!("[{:<15}] {}", region_file_name, err));
            std::fs::remove_file(region_file_path)?;
        }
    }

    Ok(())
}
