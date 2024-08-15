use crate::nbt::binary_reader::BinaryReader;
use crate::nbt::parse::parse_tag;
use crate::nbt::tag::Tag;

pub fn parse_compound_tag(reader: &mut BinaryReader) -> Vec<Tag> {
    let mut values = Vec::new();

    loop {
        let next_tag = parse_tag(reader);
        if next_tag == Tag::End {
            break;
        }
        values.push(next_tag);
    }

    values
}
