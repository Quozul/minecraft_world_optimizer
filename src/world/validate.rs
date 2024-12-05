use std::path::{Path, PathBuf};
use thiserror::Error;

pub fn validate_worlds(world_paths: &Vec<PathBuf>) -> Result<&Vec<PathBuf>, ValidateWorldError> {
    for world in world_paths {
        validate_world(world)?;
    }
    Ok(world_paths)
}

fn validate_world(world_dir: &Path) -> Result<(), ValidateWorldError> {
    if !world_dir.is_dir() {
        return Err(ValidateWorldError::NotADirectory(world_dir.to_path_buf()));
    }

    if !std::fs::exists(world_dir.join("level.dat")).unwrap_or(false) {
        return Err(ValidateWorldError::MissingLevelData);
    }

    Ok(())
}

#[derive(Error, Debug)]
pub enum ValidateWorldError {
    #[error("the provided world `{0}` directory is not a directory")]
    NotADirectory(PathBuf),
    #[error("the provided world directory is missing the `level.dat` file")]
    MissingLevelData,
}
