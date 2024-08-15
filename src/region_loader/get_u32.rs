pub fn get_u32(table: &[u8], i: usize) -> u32 {
    let bytes = [table[i], table[i + 1], table[i + 2], table[i + 3]];
    u32::from_be_bytes(bytes)
}
