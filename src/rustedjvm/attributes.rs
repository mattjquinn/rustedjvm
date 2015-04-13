use std::str;
use std::collections::HashMap;

use constant_pool::*;

pub enum Attribute {
    Code(ATTRIBUTE_Code),
}

pub struct ATTRIBUTE_Code {
    pub attr_name_idx: u16,
    pub attr_length: u16,
}

impl Attribute {
    pub fn from_bytecodes(bytecodes: &Vec<u8>, byte_idx: &mut usize,
                          constant_pool: &HashMap<u16, ConstantPoolEntry>)
                                -> Attribute {

        let attr_name_idx = (bytecodes[*byte_idx] + bytecodes[*byte_idx + 1]) as u16;
        *byte_idx = *byte_idx + 2;

        let name_constant: &CONSTANT_Utf8 = match constant_pool.get(&attr_name_idx) {
            Some(&ConstantPoolEntry::Utf8(ref s)) => s,
            Some(_) => panic!("Expected Utf8 constant at \
                                   attribute name idx: {}", attr_name_idx),
            None => panic!("No entry in constant pool at idx: {}", attr_name_idx),
        };

        match name_constant.utf8_str {
            "Code" => Attribute::Code(
                    ATTRIBUTE_Code::from_bytecodes(attr_name_idx, bytecodes, byte_idx)),
            _ => panic!("Expected \"Code\" for attribute name, encountered: {}",
                    name_constant.utf8_str),
        }
    }

    pub fn to_string(&self) -> String {
        match *self {
            Attribute::Code(ref s) => format!(
                "ATTRIBUTE_Code[attr_name_idx={}, attr_length={}]",
                        s.attr_name_idx, s.attr_length),
        }
    }
}

impl ATTRIBUTE_Code {
    pub fn from_bytecodes(attr_name_idx: u16, bytecodes: &Vec<u8>,
                          byte_idx: &mut usize) -> ATTRIBUTE_Code {

        let attr_length = bytecodes[*byte_idx..*byte_idx+4]
                .iter().fold(0, |s, &x| s + x) as u16;

        ATTRIBUTE_Code {
            attr_name_idx: attr_name_idx,
            attr_length: attr_length,
        }
    }
}
