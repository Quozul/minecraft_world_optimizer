use crate::nbt::writers::{
    size_to_i32_bytes, write_array_i32, write_array_i64, write_array_i8, write_string,
};

#[derive(PartialEq, Debug, Clone)]
pub enum Tag {
    End,
    Byte {
        name: Option<String>,
        value: i8,
    },
    Short {
        name: Option<String>,
        value: i16,
    },
    Int {
        name: Option<String>,
        value: i32,
    },
    Long {
        name: Option<String>,
        value: i64,
    },
    Float {
        name: Option<String>,
        value: f32,
    },
    Double {
        name: Option<String>,
        value: f64,
    },
    ByteArray {
        name: Option<String>,
        value: Vec<i8>,
    },
    String {
        name: Option<String>,
        value: String,
    },
    List {
        name: Option<String>,
        value: Vec<Tag>,
        tag_type: u8,
    },
    Compound {
        name: Option<String>,
        value: Vec<Tag>,
    },
    IntArray {
        name: Option<String>,
        value: Vec<i32>,
    },
    LongArray {
        name: Option<String>,
        value: Vec<i64>,
    },
}

impl Tag {
    fn get_tag_type(&self) -> u8 {
        match self {
            Tag::End => 0,
            Tag::Byte { .. } => 1,
            Tag::Short { .. } => 2,
            Tag::Int { .. } => 3,
            Tag::Long { .. } => 4,
            Tag::Float { .. } => 5,
            Tag::Double { .. } => 6,
            Tag::ByteArray { .. } => 7,
            Tag::String { .. } => 8,
            Tag::List { .. } => 9,
            Tag::Compound { .. } => 10,
            Tag::IntArray { .. } => 11,
            Tag::LongArray { .. } => 12,
        }
    }

    fn get_name(&self) -> Option<String> {
        match self {
            Tag::End => None,
            Tag::Byte { name, .. } => name.clone(),
            Tag::Short { name, .. } => name.clone(),
            Tag::Int { name, .. } => name.clone(),
            Tag::Long { name, .. } => name.clone(),
            Tag::Float { name, .. } => name.clone(),
            Tag::Double { name, .. } => name.clone(),
            Tag::ByteArray { name, .. } => name.clone(),
            Tag::String { name, .. } => name.clone(),
            Tag::List { name, .. } => name.clone(),
            Tag::Compound { name, .. } => name.clone(),
            Tag::IntArray { name, .. } => name.clone(),
            Tag::LongArray { name, .. } => name.clone(),
        }
    }

    fn serialize_name(&self) -> Vec<u8> {
        match self.get_name() {
            None => Vec::from([0, 0]),
            Some(name) => write_string(name),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.to_bytes_tag(false, false)
    }

    fn to_bytes_tag(&self, skip_name: bool, skip_tag_type: bool) -> Vec<u8> {
        let tag_type = self.get_tag_type();
        let mut base = if skip_tag_type {
            Vec::new()
        } else {
            Vec::from([tag_type])
        };

        if !skip_name && tag_type != 0 {
            base.extend(self.serialize_name());
        }

        match self {
            Tag::End => {}
            Tag::Byte { value, .. } => {
                base.extend(value.to_be_bytes());
            }
            Tag::Short { value, .. } => {
                base.extend(value.to_be_bytes());
            }
            Tag::Int { value, .. } => {
                base.extend(value.to_be_bytes());
            }
            Tag::Long { value, .. } => {
                base.extend(value.to_be_bytes());
            }
            Tag::Float { value, .. } => {
                base.extend(value.to_be_bytes());
            }
            Tag::Double { value, .. } => {
                base.extend(value.to_be_bytes());
            }
            Tag::ByteArray { value, .. } => {
                base.extend(write_array_i8(value));
            }
            Tag::String { value, .. } => {
                base.extend(write_string(value.clone()));
            }
            Tag::List {
                value, tag_type, ..
            } => {
                let mut serialized_value: Vec<u8> = Vec::from([*tag_type]);
                let size_bytes = size_to_i32_bytes(value.len());
                serialized_value.extend_from_slice(&size_bytes);
                for next_tag in value {
                    serialized_value.extend(next_tag.to_bytes_tag(true, true));
                }
                base.extend(serialized_value);
            }
            Tag::Compound { value, .. } => {
                let mut serialized_value: Vec<u8> = Vec::new();
                for next_tag in value {
                    serialized_value.extend(next_tag.to_bytes_tag(false, false));
                }
                serialized_value.extend(Tag::End.to_bytes_tag(true, false));
                base.extend(serialized_value);
            }
            Tag::IntArray { value, .. } => {
                base.extend(write_array_i32(value));
            }
            Tag::LongArray { value, .. } => {
                base.extend(write_array_i64(value));
            }
        };

        base
    }

    pub fn get_long(&self) -> Option<&i64> {
        match self {
            Tag::Long { value, .. } => Some(value),
            _ => None,
        }
    }

    pub fn get_int(&self) -> Option<&i32> {
        match self {
            Tag::Int { value, .. } => Some(value),
            _ => None,
        }
    }

    pub fn get_string(&self) -> Option<&String> {
        match self {
            Tag::String { value, .. } => Some(value),
            _ => None,
        }
    }

    pub fn find_tag(&self, name: impl ToString) -> Option<&Tag> {
        let name = name.to_string();
        match self {
            Self::Compound { value, .. } => value
                .iter()
                .find(|v| v.get_name().is_some_and(|v| v == name)),
            _ => None,
        }
    }
}
