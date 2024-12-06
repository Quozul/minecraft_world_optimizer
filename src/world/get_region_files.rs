use crate::world::validate::validate_worlds;
use std::error::Error;
use std::path::{Path, PathBuf};

pub fn get_region_files(world_paths: &Vec<PathBuf>) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let worlds = validate_worlds(world_paths)?;
    Ok(worlds
        .iter()
        .flat_map(|world| get_region_files_from_world(world))
        .collect::<Vec<_>>())
}

fn get_region_files_from_world(world_dir: &Path) -> Vec<PathBuf> {
    let mut overworld_regions = get_region_dir(world_dir.to_path_buf());
    let nether_regions = get_region_dir(world_dir.join("DIM-1"));
    let the_end_regions = get_region_dir(world_dir.join("DIM1"));

    overworld_regions.extend(nether_regions);
    overworld_regions.extend(the_end_regions);

    overworld_regions
}

fn get_region_dir(dimension_directory: PathBuf) -> Vec<PathBuf> {
    get_mca_files(dimension_directory.join("region"))
}

fn get_mca_files(region_directory: PathBuf) -> Vec<PathBuf> {
    std::fs::read_dir(region_directory)
        .map(|dir| {
            dir.flatten()
                .map(|entry| entry.path())
                // mcc files are not supported yet
                .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("mca"))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}
