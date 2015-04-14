use std::str;
use std::collections::HashMap;

use constant_pool::*;

pub enum Attribute<'a> {
    Code(ATTRIBUTE_Code<'a>),
}

pub struct ATTRIBUTE_Code<'a> {
    pub attr_name_idx: u16,
    pub attr_length: u16,
    pub max_stack: u16,
    pub max_locals: u16,
    pub code_length: usize,
    pub code_slice: &'a[u8],
    pub exception_table_length: usize,
}

impl<'a> Attribute<'a> {
    pub fn from_bytecodes(bytecodes: &'a Vec<u8>, byte_idx: &mut usize,
                          constant_pool: &HashMap<u16, ConstantPoolEntry>)
                                -> Attribute<'a> {

        let attr_name_idx = (bytecodes[*byte_idx]
                             + bytecodes[*byte_idx + 1]) as u16;
        *byte_idx = *byte_idx + 2;

        let name_constant: &CONSTANT_Utf8 =
                match constant_pool.get(&attr_name_idx) {
            Some(&ConstantPoolEntry::Utf8(ref s)) => s,
            Some(_) => panic!("Expected Utf8 constant at \
                                   attribute name idx: {}", attr_name_idx),
            None => panic!("No entry in constant pool at idx: {}", attr_name_idx),
        };

        match name_constant.utf8_str {
            "Code" => Attribute::Code(
                    ATTRIBUTE_Code::from_bytecodes(
                        attr_name_idx, bytecodes, byte_idx)),
            _ => panic!("Expected \"Code\" for attribute name, encountered: {}",
                    name_constant.utf8_str),
        }
    }

    pub fn to_string(&self) -> String {
        match *self {
            Attribute::Code(ref s) => format!("ATTRIBUTE_Code:\n\
                \t\t- attr_name_idx={}\n\
                \t\t- attr_length={}\n\
                \t\t- max_stack={}\n\
                \t\t- max_locals={}\n\
                \t\t- code_length={}\n\
                \t\t- exception_table_length={}",
                s.attr_name_idx, s.attr_length, s.max_stack,
                s.max_locals, s.code_length, s.exception_table_length),
        }
    }
}

impl<'a> ATTRIBUTE_Code<'a> {
    pub fn from_bytecodes(attr_name_idx: u16, bytecodes: &'a Vec<u8>,
                          byte_idx: &mut usize) -> ATTRIBUTE_Code<'a> {

        let attr_length = bytecodes[*byte_idx..*byte_idx+4]
                .iter().fold(0, |s, &x| s + x) as u16;
        *byte_idx = *byte_idx + 4;

        let max_stack = (bytecodes[*byte_idx]
                         + bytecodes[*byte_idx + 1]) as u16;
        *byte_idx = *byte_idx + 2;

        let max_locals = (bytecodes[*byte_idx]
                          + bytecodes[*byte_idx + 1]) as u16;
        *byte_idx = *byte_idx + 2;

        let code_length: usize = bytecodes[*byte_idx..*byte_idx+4]
                .iter().fold(0, |s, &x| s + x) as usize;
        *byte_idx = *byte_idx + 4;

        let code_start_byte = *byte_idx;
        let code_end_byte = *byte_idx + code_length;
        let code_slice: &[u8] = &bytecodes[code_start_byte..code_end_byte];
        *byte_idx = code_end_byte;

        let exception_table_length: usize = bytecodes[*byte_idx..*byte_idx+4]
            .iter().fold(0, |s, &x| s + x) as usize;
        *byte_idx = *byte_idx + 4;

        ATTRIBUTE_Code {
            attr_name_idx: attr_name_idx,
            attr_length: attr_length,
            max_stack: max_stack,
            max_locals: max_locals,
            code_length: code_length,
            code_slice: code_slice,
            exception_table_length: exception_table_length,
        }
    }
}
