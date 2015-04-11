pub struct CONSTANT_Class {
    pub name_idx: u16,
}

pub struct CONSTANT_String {
    pub string_idx: u16,
}

pub struct CONSTANT_FieldRef {
    pub class_idx: u16,
    pub name_and_type_idx: u16,
}

pub struct CONSTANT_MethodRef {
    pub class_idx: u16,
    pub name_and_type_idx: u16,
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

impl CONSTANT_String {
    pub fn from_bytecodes(bytecodes: &Vec<u8>, byte_idx: &mut usize) -> CONSTANT_String {
        let entry = CONSTANT_String {
            string_idx: (bytecodes[*byte_idx + 1] + bytecodes[*byte_idx + 2]) as u16,
        };
        *byte_idx = *byte_idx + 3;
        entry
    }
}

impl CONSTANT_FieldRef {
    pub fn from_bytecodes(bytecodes: &Vec<u8>, byte_idx: &mut usize) -> CONSTANT_FieldRef {
        let entry = CONSTANT_FieldRef {
            class_idx: (bytecodes[*byte_idx + 1] + bytecodes[*byte_idx + 2]) as u16,
            name_and_type_idx: (bytecodes[*byte_idx + 3] + bytecodes[*byte_idx + 4]) as u16,
        };
        *byte_idx = *byte_idx + 5;
        entry
    }
}

impl CONSTANT_MethodRef {
    pub fn from_bytecodes(bytecodes: &Vec<u8>, byte_idx: &mut usize) -> CONSTANT_MethodRef {
        let entry = CONSTANT_MethodRef {
            class_idx: (bytecodes[*byte_idx + 1] + bytecodes[*byte_idx + 2]) as u16,
            name_and_type_idx: (bytecodes[*byte_idx + 3] + bytecodes[*byte_idx + 4]) as u16,
        };
        *byte_idx = *byte_idx + 5;
        entry
    }
}

