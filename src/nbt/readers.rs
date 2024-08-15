use std::string::FromUtf8Error;

pub fn read_i8(raw: &[u8], index: &mut usize) -> i8 {
    let integer = i8::from_be_bytes([raw[*index]]);
    *index += 1_usize;
    integer
}

pub fn read_i16(raw: &[u8], index: &mut usize) -> i16 {
    let integer = i16::from_be_bytes([raw[*index], raw[*index + 1]]);
    *index += 2_usize;
    integer
}

pub fn read_u16(raw: &[u8], index: &mut usize) -> u16 {
    let integer = u16::from_be_bytes([raw[*index], raw[*index + 1]]);
    *index += 2_usize;
    integer
}

pub fn read_i32(raw: &[u8], index: &mut usize) -> i32 {
    let integer = i32::from_be_bytes([
        raw[*index],
        raw[*index + 1],
        raw[*index + 2],
        raw[*index + 3],
    ]);
    *index += 4_usize;
    integer
}

pub fn read_i64(raw: &[u8], index: &mut usize) -> i64 {
    let integer = i64::from_be_bytes([
        raw[*index],
        raw[*index + 1],
        raw[*index + 2],
        raw[*index + 3],
        raw[*index + 4],
        raw[*index + 5],
        raw[*index + 6],
        raw[*index + 7],
    ]);
    *index += 8_usize;
    integer
}

pub fn read_f32(raw: &[u8], index: &mut usize) -> f32 {
    let integer = f32::from_be_bytes([
        raw[*index],
        raw[*index + 1],
        raw[*index + 2],
        raw[*index + 3],
    ]);
    *index += 4_usize;
    integer
}

pub fn read_f64(raw: &[u8], index: &mut usize) -> f64 {
    let integer = f64::from_be_bytes([
        raw[*index],
        raw[*index + 1],
        raw[*index + 2],
        raw[*index + 3],
        raw[*index + 4],
        raw[*index + 5],
        raw[*index + 6],
        raw[*index + 7],
    ]);
    *index += 8_usize;
    integer
}

pub fn read_string(raw: &[u8], index: &mut usize) -> Result<String, FromUtf8Error> {
    let size = read_u16(raw, index);
    let start = *index;
    let end = start + size as usize;
    let string_bytes = &raw[start..end];
    *index = end;
    String::from_utf8(Vec::from(string_bytes))
}

// Special readers
pub fn read_name(raw: &[u8], index: &mut usize) -> Option<String> {
    read_string(raw, index).ok().filter(|s| !s.is_empty())
}

pub fn read_type(raw: &[u8], index: &mut usize) -> u8 {
    read_i8(raw, index) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_string() {
        let data = &[0, 5, 72, 69, 76, 76, 79];
        let mut index = 0_usize;
        let parsed = read_string(data, &mut index).unwrap();

        assert_eq!(parsed, "HELLO");
    }
}
