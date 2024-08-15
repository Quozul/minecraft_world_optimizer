use crate::nbt::readers::{
    read_f32, read_f64, read_i16, read_i32, read_i64, read_i8, read_name, read_string, read_type,
};
use crate::nbt::tag::Tag;

pub fn parse_tag(raw: &[u8], index: &mut usize) -> Tag {
    let tag_type = read_type(raw, index);
    parse_of_type(raw, index, tag_type, false)
}

fn parse_of_type(raw: &[u8], index: &mut usize, tag_type: u8, skip_name: bool) -> Tag {
    let name = if skip_name || tag_type == 0 {
        None
    } else {
        read_name(raw, index)
    };

    match tag_type {
        0 => Tag::End,
        1 => {
            let value = read_i8(raw, index);
            Tag::Byte { name, value }
        }
        2 => {
            let value = read_i16(raw, index);
            Tag::Short { name, value }
        }
        3 => {
            let value = read_i32(raw, index);
            Tag::Int { name, value }
        }
        4 => {
            let value = read_i64(raw, index);
            Tag::Long { name, value }
        }
        5 => {
            let value = read_f32(raw, index);
            Tag::Float { name, value }
        }
        6 => {
            let value = read_f64(raw, index);
            Tag::Double { name, value }
        }
        7 => {
            let value = parse_array(raw, index, read_i8);
            Tag::ByteArray { name, value }
        }
        8 => {
            let value = read_string(raw, index).unwrap_or_default();
            Tag::String { name, value }
        }
        9 => {
            let (tag_type, value) = parse_list_tag(raw, index);
            Tag::List {
                name,
                value,
                tag_type,
            }
        }
        10 => {
            let value = parse_compound_tag(raw, index);
            Tag::Compound { name, value }
        }
        11 => {
            let value = parse_array(raw, index, read_i32);
            Tag::IntArray { name, value }
        }
        12 => {
            let value = parse_array(raw, index, read_i64);
            Tag::LongArray { name, value }
        }
        _ => panic!("Unsupported tag type {tag_type}"),
    }
}

fn parse_list_tag(raw: &[u8], index: &mut usize) -> (u8, Vec<Tag>) {
    let mut values = Vec::new();

    let tag_type = read_type(raw, index);
    let list_length = read_i32(raw, index);
    if list_length <= 0 && tag_type == 0 {
        return (tag_type, values);
    }

    for _ in 0..list_length {
        let next_tag = parse_of_type(raw, index, tag_type, true);
        values.push(next_tag);
    }

    (tag_type, values)
}

fn parse_compound_tag(raw: &[u8], index: &mut usize) -> Vec<Tag> {
    let mut values = Vec::new();

    loop {
        let next_tag = parse_tag(raw, index);
        if next_tag == Tag::End {
            break;
        }
        values.push(next_tag);
    }

    values
}

fn parse_array<T>(raw: &[u8], index: &mut usize, parser: fn(&[u8], &mut usize) -> T) -> Vec<T> {
    let size = read_i32(raw, index);
    let mut values = Vec::new();

    for _ in 0..size {
        let next_tag = parser(raw, index);
        values.push(next_tag);
    }

    values
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world() {
        let data = include_bytes!("../../test_files/hello_world.nbt");
        let mut index = 0_usize;
        let result = parse_tag(data, &mut index);

        assert_eq!(
            result,
            Tag::Compound {
                name: Some(String::from("hello world")),
                value: Vec::from([Tag::String {
                    name: Some(String::from("name")),
                    value: String::from("Bananrama"),
                }])
            }
        );

        let serialized = result.to_bytes();
        assert_eq!(serialized, data);
    }

    #[test]
    fn test_bigtest() {
        let data = include_bytes!("../../test_files/bigtest.nbt");

        let mut index = 0_usize;
        let result = parse_tag(data, &mut index);

        // Build the ByteArray
        let mut value = Vec::new();
        for n in 0..1000 {
            let r = ((n * n * 255 + n * 7) % 100) as i8;
            value.push(r);
        }

        let expected = Tag::Compound {
            name: Some(String::from("Level")),
            value: Vec::from([
                Tag::Long {
                    name: Some(String::from("longTest")),
                    value: 9223372036854775807,
                },
                Tag::Short {
                    name: Some(String::from("shortTest")),
                    value: 32767,
                },
                Tag::String {
                    name: Some(String::from("stringTest")),
                    value: String::from("HELLO WORLD THIS IS A TEST STRING ÅÄÖ!"),
                },
                Tag::Float {
                    name: Some(String::from("floatTest")),
                    value: 0.498_231_470_584_869_38_f32,
                },
                Tag::Int {
                    name: Some(String::from("intTest")),
                    value: 2147483647,
                },
                Tag::Compound {
                    name: Some(String::from("nested compound test")),
                    value: Vec::from([
                        Tag::Compound {
                            name: Some(String::from("ham")),
                            value: Vec::from([
                                Tag::String {
                                    name: Some(String::from("name")),
                                    value: String::from("Hampus"),
                                },
                                Tag::Float {
                                    name: Some(String::from("value")),
                                    value: 0.75,
                                }
                            ]),
                        },
                        Tag::Compound {
                            name: Some(String::from("egg")),
                            value: Vec::from([
                                Tag::String {
                                    name: Some(String::from("name")),
                                    value: String::from("Eggbert"),
                                },
                                Tag::Float {
                                    name: Some(String::from("value")),
                                    value: 0.5,
                                }
                            ]),
                        },
                    ]),
                },
                Tag::List {
                    name: Some(String::from("listTest (long)")),
                    tag_type: 4,
                    value: Vec::from([
                        Tag::Long {
                            name: None,
                            value: 11,
                        },
                        Tag::Long {
                            name: None,
                            value: 12,
                        },
                        Tag::Long {
                            name: None,
                            value: 13,
                        },
                        Tag::Long {
                            name: None,
                            value: 14,
                        },
                        Tag::Long {
                            name: None,
                            value: 15,
                        },
                    ]),
                },
                Tag::List {
                    name: Some(String::from("listTest (compound)")),
                    tag_type: 10,
                    value: Vec::from([
                        Tag::Compound {
                            name: None,
                            value: Vec::from([
                                Tag::String {
                                    name: Some(String::from("name")),
                                    value: String::from("Compound tag #0"),
                                },
                                Tag::Long {
                                    name: Some(String::from("created-on")),
                                    value: 1264099775885,
                                },
                            ]),
                        },
                        Tag::Compound {
                            name: None,
                            value: Vec::from([
                                Tag::String {
                                    name: Some(String::from("name")),
                                    value: String::from("Compound tag #1"),
                                },
                                Tag::Long {
                                    name: Some(String::from("created-on")),
                                    value: 1264099775885,
                                },
                            ]),
                        },
                    ]),
                },
                Tag::Byte {
                    name: Some(String::from("byteTest")),
                    value: 127,
                },
                Tag::ByteArray {
                    name: Some(String::from("byteArrayTest (the first 1000 values of (n*n*255+n*7)%100, starting with n=0 (0, 62, 34, 16, 8, ...))")),
                    value,
                },
                Tag::Double {
                    name: Some(String::from("doubleTest")),
                    value: 0.493_128_713_218_231_48_f64,
                },
            ]),
        };

        assert_eq!(result, expected);

        let serialized = result.to_bytes();
        assert_eq!(serialized, data);
    }
}
