pub enum CompressionScheme {
    Gzip,
    Zlib,
}

impl CompressionScheme {
    pub fn from_u8(byte: u8) -> Result<Self, &'static str> {
        match byte {
            1 => Ok(CompressionScheme::Gzip),
            2 => Ok(CompressionScheme::Zlib),
            _ => Err("Unsupported compression scheme"),
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            CompressionScheme::Gzip => 1,
            CompressionScheme::Zlib => 2,
        }
    }
}
