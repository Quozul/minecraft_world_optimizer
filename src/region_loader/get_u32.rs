pub fn get_u32(table: &[u8], index: usize) -> u32 {
    let bytes = [
        table[index],
        table[index + 1],
        table[index + 2],
        table[index + 3],
    ];
    u32::from_be_bytes(bytes)
}
