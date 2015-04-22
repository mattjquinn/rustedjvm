use std::collections::HashMap;

use attributes::*;
use constants::*;
use std::string;

pub struct Method<'a> {
    pub access_flags: u16,
    pub name_idx: u16,
    pub name: &'a str,
    pub descriptor_idx: u16,
    pub attrs_count: u16,
    pub attributes: Vec<Attribute<'a>>,
}

impl<'a> Method<'a> {
    pub fn from_bytecodes(bytecodes: &'a Vec<u8>, byte_idx: &mut usize,
                          constant_pool: &HashMap<u16,
                          ConstantPoolEntry<'a>>) -> Method<'a> {

        let access_flags = (bytecodes[*byte_idx]
                            + bytecodes[*byte_idx + 1]) as u16;
        let name_idx = (bytecodes[*byte_idx + 2]
                        + bytecodes[*byte_idx + 3]) as u16;
        let descriptor_idx = (bytecodes[*byte_idx + 4]
                              + bytecodes[*byte_idx + 5]) as u16;
        let attrs_count = (bytecodes[*byte_idx + 6]
                           + bytecodes[*byte_idx + 7]) as u16;
        *byte_idx = *byte_idx + 8;

        let name = match constant_pool.get(&name_idx) {
            Some(&ConstantPoolEntry::Utf8(ref e)) => e.utf8_str,
            Some(_) | None => panic!("[ERROR] Can't find name at index {} \
                            in constant pool.", name_idx),
        };

        let mut attributes = Vec::new();
        for n in 0 .. attrs_count {
            let attr = Attribute::from_bytecodes(
                bytecodes, byte_idx, constant_pool);
            attributes.push(attr);
        };

        Method {
            access_flags: access_flags,
            name_idx: name_idx,
            name: name,
            descriptor_idx: descriptor_idx,
            attrs_count: attrs_count,
            attributes: attributes,
        }
    }

    pub fn to_string(&self) -> String {
        let mut string_rep = format!("Method:\n\
                \t- access_flags=0x{:x}\n\
                \t- name_idx={}\n\
                \t- descriptor_idx={}\n\
                \t- attrs_count={}\n",
                self.access_flags, self.name_idx, self.descriptor_idx,
                self.attrs_count);

        for attr in self.attributes.iter() {
            string_rep = string_rep + &format!("\tAttribute:{}",
                                               attr.to_string());
        }

        string_rep
    }
}
