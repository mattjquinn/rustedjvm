use std::str;
use std::collections::HashMap;

use constants::*;
use exceptions::*;

pub enum Attribute<'a> {
    Code(ATTRIBUTE_Code<'a>),
    LineNumberTable(ATTRIBUTE_LineNumberTable),
    SourceFile(ATTRIBUTE_SourceFile),
}

pub struct ATTRIBUTE_Code<'a> {
    pub attr_name_idx: u16,
    pub attr_length: u16,
    pub max_stack: u16,
    pub max_locals: u16,
    pub code_length: usize,
    pub code_slice: &'a[u8],
    pub exception_table_length: usize,
    pub exception_table: Vec<ExceptionTableEntry>,
    pub attribute_count: u16,
    pub attributes: Vec<Attribute<'a>>,
}

pub struct ATTRIBUTE_LineNumberTable {
    pub attr_name_idx: u16,
    pub attr_length: u16,
    pub line_number_table_length: u16,
    pub line_nbr_table_entries: Vec<LineNumberTableEntry>,
}

pub struct ATTRIBUTE_SourceFile {
    pub attr_name_idx: u16,
}

pub struct LineNumberTableEntry {
    pub start_pc: u16,
    pub line_nbr: u16,
}

impl LineNumberTableEntry {
    pub fn to_string(&self) -> String {
        format!("LineNumberTableEntry:\n\
            \t\t\t\t- start_pc={}\n\
            \t\t\t\t- line_nbr={}\n", self.start_pc, self.line_nbr)
    }
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
                        attr_name_idx, bytecodes, byte_idx, constant_pool)),
            "LineNumberTable" => Attribute::LineNumberTable(
                    ATTRIBUTE_LineNumberTable::from_bytecodes(
                        attr_name_idx, bytecodes, byte_idx, constant_pool)),
            "SourceFile" => Attribute::SourceFile(
                    ATTRIBUTE_SourceFile::from_bytecodes(
                        attr_name_idx, bytecodes, byte_idx, constant_pool)),
            _ => panic!("Unexpected attribute name encountered: {}",
                    name_constant.utf8_str),
        }
    }

    pub fn to_string(&self) -> String {
        match *self {
            Attribute::Code(ref s) => {
                let mut string_rep = format!("ATTRIBUTE_Code:\n\
                    \t\t- attr_name_idx={}\n\
                    \t\t- attr_length={}\n\
                    \t\t- max_stack={}\n\
                    \t\t- max_locals={}\n\
                    \t\t- code_length={}\n\
                    \t\t- exception_table_length={}\n\
                    \t\t- attribute_count={}\n",
                    s.attr_name_idx, s.attr_length, s.max_stack,
                    s.max_locals, s.code_length, s.exception_table_length,
                    s.attribute_count);

                for entry in s.exception_table.iter() {
                    string_rep = string_rep + &format!(
                        "\t\tException Handler:{}", entry.to_string());
                }

                for attr in s.attributes.iter() {
                    string_rep = string_rep + &format!(
                        "\t\tAttribute:{}", attr.to_string());
                }

                string_rep
            },
            Attribute::LineNumberTable(ref s) => {
                let mut string_rep = format!("ATTRIBUTE_LineNumberTable:\n\
                    \t\t\t- attr_name_idx={}\n\
                    \t\t\t- attr_length={}\n\
                    \t\t\t- line_number_table_length={}\n",
                    s.attr_name_idx, s.attr_length, s.line_number_table_length);

                for entry in s.line_nbr_table_entries.iter() {
                    string_rep = string_rep + &format!(
                        "\t\t\t{}", entry.to_string());
                }

                string_rep
            },
            Attribute::SourceFile(ref s) => {
                format!("ATTRIBUTE_SourceFile:\n\
                    \t- attr_name_idx={}",
                    s.attr_name_idx)
            }
        }
    }
}

impl<'a> ATTRIBUTE_Code<'a> {
    pub fn from_bytecodes(attr_name_idx: u16, bytecodes: &'a Vec<u8>,
                          byte_idx: &mut usize,
                          constant_pool: &HashMap<u16, ConstantPoolEntry>)
                          -> ATTRIBUTE_Code<'a> {

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

        let exception_table_length: usize = (bytecodes[*byte_idx]
                                             + bytecodes[*byte_idx + 1]) as usize;
        *byte_idx = *byte_idx + 2;

        let mut exception_table = Vec::new();
        for n in 0 .. exception_table_length {
            let entry = ExceptionTableEntry::from_bytecodes(bytecodes, byte_idx);
            exception_table.push(entry);
        }

        let attribute_count: u16 = (bytecodes[*byte_idx]
                                    + bytecodes[*byte_idx + 1]) as u16;
        *byte_idx = *byte_idx + 2;

        let mut attributes: Vec<Attribute> = Vec::new();
        for n in 0 .. attribute_count {
            let attr = Attribute::from_bytecodes(
                    bytecodes, byte_idx, constant_pool);
            attributes.push(attr);
        }

        ATTRIBUTE_Code {
            attr_name_idx: attr_name_idx,
            attr_length: attr_length,
            max_stack: max_stack,
            max_locals: max_locals,
            code_length: code_length,
            code_slice: code_slice,
            exception_table_length: exception_table_length,
            exception_table: exception_table,
            attribute_count: attribute_count,
            attributes: attributes,
        }
    }
}

impl ATTRIBUTE_LineNumberTable {
    pub fn from_bytecodes(attr_name_idx: u16, bytecodes: &Vec<u8>,
                          byte_idx: &mut usize,
                          constant_pool: &HashMap<u16, ConstantPoolEntry>)
                          -> ATTRIBUTE_LineNumberTable {

        let attr_length = bytecodes[*byte_idx..*byte_idx+4]
                .iter().fold(0, |s, &x| s + x) as u16;
        *byte_idx = *byte_idx + 4;

        let line_number_table_length: u16 = (bytecodes[*byte_idx]
                + bytecodes[*byte_idx + 1]) as u16;
        *byte_idx = *byte_idx + 2;

        let mut line_nbr_table_entries: Vec<LineNumberTableEntry> = Vec::new();
        for n in 0 .. line_number_table_length {
            let start_pc: u16 = (bytecodes[*byte_idx]
                                 + bytecodes[*byte_idx + 1]) as u16;
            *byte_idx = *byte_idx + 2;

            let line_nbr: u16 = (bytecodes[*byte_idx]
                                 + bytecodes[*byte_idx + 1]) as u16;
            *byte_idx = *byte_idx + 2;

            line_nbr_table_entries.push(LineNumberTableEntry {
                start_pc: start_pc,
                line_nbr: line_nbr,
            });
        }

        ATTRIBUTE_LineNumberTable {
            attr_name_idx: attr_name_idx,
            attr_length: attr_length,
            line_number_table_length: line_number_table_length,
            line_nbr_table_entries: line_nbr_table_entries,
        }
    }
}

impl ATTRIBUTE_SourceFile {
    pub fn from_bytecodes(attr_name_idx: u16, bytecodes: &Vec<u8>,
                          byte_idx: &mut usize,
                          constant_pool: &HashMap<u16, ConstantPoolEntry>)
                          -> ATTRIBUTE_SourceFile {
        ATTRIBUTE_SourceFile {
            attr_name_idx: attr_name_idx,
        }
    }
}
