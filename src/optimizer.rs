use crate::region_loader::chunk::Chunk;
use crate::region_loader::region::Region;
use indicatif::ProgressBar;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

fn try_read_bytes(file_path: &PathBuf) -> Result<Vec<u8>, std::io::Error> {
    let mut buf = Vec::<u8>::new();
    File::open(file_path).and_then(|mut file| file.read_to_end(&mut buf))?;
    Ok(buf)
}

pub fn optimize_region_file(
    region_file_path: PathBuf,
    pb: &ProgressBar,
) -> Result<(), std::io::Error> {
    let buf = try_read_bytes(&region_file_path)?;

    match Region::from_bytes(&buf) {
        Ok(mut region) => {
            // Read the first chunk
            let mut chunks_to_delete = Vec::new();
            for chunk in region.get_chunks() {
                if should_delete_chunk(chunk) {
                    chunks_to_delete.push(chunk.clone());
                }
            }
            for chunk in &chunks_to_delete {
                region.remove_chunk(chunk);
            }

            if region.is_empty() {
                pb.println("Region file is now empty and not needed anymore");
                std::fs::remove_file(region_file_path)?;
                return Ok(());
            }

            pb.println(format!("{} chunks deleted", chunks_to_delete.len()));

            let bytes = region.to_bytes();
            std::fs::write(region_file_path, bytes)?;
        }
        Err(_) => {
            pb.println("Invalid region file");
            std::fs::remove_file(region_file_path)?;
        }
    }

    Ok(())
}

fn should_delete_chunk(chunk: &Chunk) -> bool {
    // The InhabitedTime value seems to be incremented for all 8 chunks around a player (including the one the player is standing in)
    let inhabited_time = chunk
        .nbt
        .find_tag("InhabitedTime")
        .and_then(|tag| tag.get_long());
    let status = chunk
        .nbt
        .find_tag("Status")
        .and_then(|tag| tag.get_string());

    // No player has walked in this chunk
    if let Some(inhabited_time) = inhabited_time {
        if *inhabited_time == 0 {
            return true;
        }
    }

    // Not fully generated chunk
    if let Some(status) = status {
        let org = status.as_str();
        let comp = "minecraft:full";
        let eq = org == comp;
        if !eq {
            return true;
        }
    }

    false
}
