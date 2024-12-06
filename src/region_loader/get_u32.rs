pub fn get_u32(table: &[u8], index: usize) -> u32 {
    u32::from_be_bytes(table[index..index + 4].try_into().unwrap())
}
