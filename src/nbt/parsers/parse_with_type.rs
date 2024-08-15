use crate::nbt::binary_reader::BinaryReader;
use crate::nbt::parsers::parse_compound_tag::parse_compound_tag;
use crate::nbt::parsers::parse_list_tag::parse_list_tag;
use crate::nbt::tag::Tag;

pub fn parse_with_type(reader: &mut BinaryReader, tag_type: u8, skip_name: bool) -> Tag {
    let name = if skip_name || tag_type == 0 {
        None
    } else {
        reader.read_name()
    };

    match tag_type {
        0 => Tag::End,
        1 => {
            let value = reader.read_i8();
            Tag::Byte { name, value }
        }
        2 => {
            let value = reader.read_i16();
            Tag::Short { name, value }
        }
        3 => {
            let value = reader.read_i32();
            Tag::Int { name, value }
        }
        4 => {
            let value = reader.read_i64();
            Tag::Long { name, value }
        }
        5 => {
            let value = reader.read_f32();
            Tag::Float { name, value }
        }
        6 => {
            let value = reader.read_f64();
            Tag::Double { name, value }
        }
        7 => {
            let value = reader.read_byte_array();
            Tag::ByteArray { name, value }
        }
        8 => {
            let value = reader.read_string().unwrap_or_default();
            Tag::String { name, value }
        }
        9 => {
            let (tag_type, value) = parse_list_tag(reader);
            Tag::List {
                name,
                value,
                tag_type,
            }
        }
        10 => {
            let value = parse_compound_tag(reader);
            Tag::Compound { name, value }
        }
        11 => {
            let value = reader.read_int_array();
            Tag::IntArray { name, value }
        }
        12 => {
            let value = reader.read_long_array();
            Tag::LongArray { name, value }
        }
        _ => panic!("Unsupported tag type {tag_type}"),
    }
}
