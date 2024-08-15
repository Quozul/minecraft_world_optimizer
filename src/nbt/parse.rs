use crate::nbt::binary_reader::BinaryReader;
use crate::nbt::parsers::parse_with_type::parse_with_type;
use crate::nbt::tag::Tag;

pub fn parse_tag(reader: &mut BinaryReader) -> Tag {
    let tag_type = reader.read_type();
    parse_with_type(reader, tag_type, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world() {
        let data = include_bytes!("../../test_files/hello_world.nbt");
        let mut reader = BinaryReader::new(data);
        let result = parse_tag(&mut reader);

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
        let mut reader = BinaryReader::new(data);
        let result = parse_tag(&mut reader);

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
