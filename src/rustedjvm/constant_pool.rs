pub struct CONSTANT_Class {
    pub name_idx: u16,
}

impl CONSTANT_Class {
    pub fn from_bytecodes(bytecodes: &Vec<u8>, byte_idx: &mut usize) -> CONSTANT_Class {
        let entry = CONSTANT_Class {
            name_idx: (bytecodes[*byte_idx + 1] + bytecodes[*byte_idx + 2]) as u16,
        };
        *byte_idx = *byte_idx + 3;
        entry
    }
}

pub struct CONSTANT_String {
    pub string_idx: u16,
}

impl CONSTANT_String {
    pub fn from_bytecodes(bytecodes: &Vec<u8>, byte_idx: &mut usize) -> CONSTANT_String {
        let entry = CONSTANT_String {
            string_idx: (bytecodes[*byte_idx + 1] + bytecodes[*byte_idx + 2]) as u16,
        };
        *byte_idx = *byte_idx + 3;
        entry
    }
}
