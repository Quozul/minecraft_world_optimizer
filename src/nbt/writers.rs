fn size_to_u16_bytes(size: usize) -> [u8; 2] {
    (size as u16).to_be_bytes()
}

pub fn size_to_i32_bytes(size: usize) -> [u8; 4] {
    (size as i32).to_be_bytes()
}

fn create_initial_buffer(size: usize) -> Vec<u8> {
    let mut buffer = Vec::new();
    buffer.extend_from_slice(&size_to_i32_bytes(size));
    buffer
}

pub fn write_string(input: String) -> Vec<u8> {
    let mut buffer = Vec::new();
    buffer.extend_from_slice(&size_to_u16_bytes(input.len()));
    buffer.extend(input.as_bytes());
    buffer
}

pub fn write_array_i8(input: &[i8]) -> Vec<u8> {
    let mut buffer = create_initial_buffer(input.len());
    let bytes = input
        .iter()
        .flat_map(|e| e.to_be_bytes())
        .collect::<Vec<u8>>();
    buffer.extend(bytes);
    buffer
}

pub fn write_array_i32(input: &[i32]) -> Vec<u8> {
    let mut buffer = create_initial_buffer(input.len());
    let bytes = input
        .iter()
        .flat_map(|e| e.to_be_bytes())
        .collect::<Vec<u8>>();
    buffer.extend(bytes);
    buffer
}

pub fn write_array_i64(input: &[i64]) -> Vec<u8> {
    let mut buffer = create_initial_buffer(input.len());
    let bytes = input
        .iter()
        .flat_map(|e| e.to_be_bytes())
        .collect::<Vec<u8>>();
    buffer.extend(bytes);
    buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_write() {
        let parsed = write_string("HELLO".to_string());

        assert_eq!(parsed, &[0, 5, 72, 69, 76, 76, 79]);
    }
}
