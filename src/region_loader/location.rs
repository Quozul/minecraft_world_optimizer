#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Location {
    offset: u32,
    size: u8,
    timestamp: u32,
}

impl Location {
    /// Creates a new location, takes the real offset and size as input
    /// Meaning the offset and sizes must be a multiple of 4096
    pub fn new(offset: u32, size: u32, timestamp: u32) -> Result<Self, &'static str> {
        if offset % 4096 != 0 || size % 4096 != 0 {
            return Err("Offset and Size must be a multiple of 4096");
        }
        let size_div = size / 4096;
        if size_div > u8::MAX as u32 {
            return Err("Size too large for u8 after division by 4096");
        }

        Ok(Self {
            offset: offset / 4096,
            size: size_div as u8,
            timestamp,
        })
    }

    /// Creates a new location from the raw data contained in the region location and timestamp tables
    pub fn from_bytes(l: u32, timestamp: u32) -> Self {
        let (offset, size) = chunk_location(l);
        Self {
            offset,
            size,
            timestamp,
        }
    }

    /// If the offset and size are both 0, then the chunk at that location hasn't been generated yet.
    pub fn is_valid(&self) -> bool {
        self.size != 0 && self.offset != 0
    }

    pub fn to_location_bytes(self) -> [u8; 4] {
        let offset = (self.offset) & 0xFFFFFF;
        let location = (offset << 8) | self.size as u32;
        location.to_be_bytes()
    }

    pub fn to_timestamp_bytes(self) -> [u8; 4] {
        self.timestamp.to_be_bytes()
    }

    pub fn get_offset(&self) -> u32 {
        self.offset * 4096
    }

    pub fn get_timestamp(&self) -> u32 {
        self.timestamp
    }

    #[cfg(test)]
    fn get_size(&self) -> u32 {
        self.size as u32 * 4096
    }
}

fn chunk_location(l: u32) -> (u32, u8) {
    let offset = (l >> 8) & 0xFFFFFF;
    let size = l & 0xFF;
    (offset, size as u8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_location() {
        let location = Location::new(8192, 4096, 0).unwrap();

        assert_eq!(location.get_offset(), 8192_u32);
        assert_eq!(location.get_size(), 4096_u32);
    }

    #[test]
    fn test_chunk_function() {
        let l = 0x00000201;

        let location = Location::from_bytes(l, 0);

        assert_eq!(location.get_offset(), 8192_u32);
        assert_eq!(location.get_size(), 4096_u32);
        assert_eq!(location.get_timestamp(), 0);
    }

    #[test]
    fn test_to_bytes() {
        let location = Location::new(8192, 4096, 0).unwrap();

        assert_eq!(location.to_location_bytes(), [0, 0, 2, 1]);
        assert_eq!(location.to_timestamp_bytes(), [0, 0, 0, 0]);
    }
}
