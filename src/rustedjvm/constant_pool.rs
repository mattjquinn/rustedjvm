pub struct CONSTANT_Class {
    pub name_idx: u8,
}

impl CONSTANT_Class {
    pub fn from_bytecodes(bytecodes: &Vec<u8>, byte_idx: &mut usize) -> CONSTANT_Class {
        let entry = CONSTANT_Class {
            name_idx: bytecodes[*byte_idx + 1] + bytecodes[*byte_idx + 2],
        };
        *byte_idx = *byte_idx + 3;
        entry
    }
}
