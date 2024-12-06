use crate::region_loader::chunk_loader::chunk::Chunk;
use crate::region_loader::get_u32::get_u32;
use crate::region_loader::location::Location;
use flate2::Compression;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(PartialEq, Debug)]
pub struct Region {
    chunks: Vec<Chunk>,
}

impl Region {
    pub fn from_file_name(file_name: &PathBuf) -> Result<Self, &'static str> {
        let bytes = try_read_bytes(file_name).map_err(|_| "Error while reading the file")?;
        Region::from_bytes(&bytes)
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, &'static str> {
        let mut chunks = Vec::with_capacity(1024);
        if bytes.len() < 8192 {
            return Err("Cannot read header of region file");
        }

        let location_table = &bytes[0..4096];
        let timestamp_table = &bytes[4096..8192];

        for i in (0..4096).step_by(4) {
            let l = get_u32(location_table, i);
            let timestamp = get_u32(timestamp_table, i);
            let location = Location::from_bytes(l, timestamp);

            if location.is_valid() {
                if let Ok(chunk) = Chunk::from_location(bytes, location) {
                    chunks.push(chunk);
                }
                // Else, we choose to not load the chunk and loose it because it is invalid
                // FIXME: We might not want to loose the chunk if the compression scheme is an unsupported type (eg. LZ4 since 24w04a or custom algorithm since 24w05a)
            }
        }

        Ok(Self { chunks })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut data = Vec::new();
        let mut location_table = [0_u8; 4096];
        let mut timestamp_table = [0_u8; 4096];

        for chunk in &self.chunks {
            // Serialize the chunk to bytes
            let mut serialized = chunk.to_bytes(Compression::best());
            align_vec_size(&mut serialized);

            // Build the new location
            let new_position = (data.len() + 8192) as u32;
            let new_size = serialized.len() as u32;
            let original_timestamp = chunk.location.get_timestamp();
            let new_location = Location::new(new_position, new_size, original_timestamp);

            let chunk_position = chunk.get_position();
            if let (Ok(new_location), Ok((x, z))) = (new_location, chunk_position) {
                // Add the location to the header table
                let position_in_table = get_position_in_table(x, z);

                // Append to the location table
                let location_bytes = new_location.to_location_bytes();
                location_table[position_in_table..(4 + position_in_table)]
                    .copy_from_slice(&location_bytes);

                // Append to the timestamp table
                let timestamp_bytes = new_location.to_timestamp_bytes();
                timestamp_table[position_in_table..(4 + position_in_table)]
                    .copy_from_slice(&timestamp_bytes);
            }
            // Else, the chunk is probably invalid, we can ignore it
            // FIXME: We might not want to loose the corrupted chunk

            data.extend(serialized);
        }

        let mut result = Vec::new();
        result.extend_from_slice(&location_table);
        result.extend_from_slice(&timestamp_table);
        result.extend(data);
        result
    }

    pub fn get_chunks(&self) -> &Vec<Chunk> {
        &self.chunks
    }

    pub fn get_chunk_count(&self) -> usize {
        self.chunks.len()
    }

    pub fn remove_chunk_by_index(&mut self, index: usize) {
        self.chunks.remove(index);
    }

    pub fn is_empty(&self) -> bool {
        self.chunks.is_empty()
    }
}

fn align_vec_size(vec: &mut Vec<u8>) {
    let aligned_size = ((vec.len() + 4095) / 4096) * 4096;
    vec.resize(aligned_size, 0);
}

fn get_position_in_table(x: i32, z: i32) -> usize {
    (4 * ((x & 31) + (z & 31) * 32)) as usize
}

fn try_read_bytes(file_path: &PathBuf) -> std::io::Result<Vec<u8>> {
    let mut buf = Vec::<u8>::new();
    File::open(file_path).and_then(|mut file| file.read_to_end(&mut buf))?;
    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_align_vec_size() {
        let mut vec_500 = vec![0; 500];
        align_vec_size(&mut vec_500);
        assert_eq!(4096, vec_500.len());

        let mut vec_4096 = vec![0; 4096];
        align_vec_size(&mut vec_4096);
        assert_eq!(4096, vec_4096.len());

        let mut vec_4097 = vec![0; 4097];
        align_vec_size(&mut vec_4097);
        assert_eq!(8192, vec_4097.len());
    }

    #[test]
    fn test_small_region() {
        let original_bytes = include_bytes!("../../test_files/r.-1.-1.mca");

        // Parse the region file
        let original_parsed_region_file = Region::from_bytes(original_bytes).unwrap();
        let serialized_bytes = original_parsed_region_file.to_bytes();

        // Wa cannot validate the header as the compression and chunk order in the payload may differ
        // resulting in a modification of the offset bytes, so as long as the re-parsed region file is
        // the same as the parsed original, we should be fine

        // Try parsing again the serialized region file and check if both still have the same chunk data
        let parsed_again = Region::from_bytes(&serialized_bytes).unwrap();

        let original_chunks = original_parsed_region_file.get_chunks();
        let parsed_chunks = parsed_again.get_chunks();
        assert_eq!(parsed_chunks.len(), original_chunks.len());

        // Assert the chunk data is unchanged
        for i in 0..original_chunks.len() {
            let original_chunk = &original_chunks[i];
            let parsed_chunk = &parsed_chunks[i];
            // We cannot check for equality on the location since it might have different offset and size
            assert_eq!(original_chunk.nbt, parsed_chunk.nbt);
        }
    }
}
