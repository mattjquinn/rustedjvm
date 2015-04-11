use std::str;

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

// Lifetime must be made explict
// here because utf8_str is only valid
// for as long as the underlying bytecode array lives.
pub struct CONSTANT_Utf8<'a> {
    pub utf8_str: &'a str,
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

impl<'a> CONSTANT_Utf8<'a> {

    // The explict 'a lifetime tags link the bytecode array with the returned struct,
    // because the string slice reference is only valid as long as the bytecode array is alive.
    pub fn from_bytecodes(bytecodes: &'a Vec<u8>, byte_idx: &mut usize) -> CONSTANT_Utf8<'a> {
        // Bytecodes are u8, but slicing requires arguments of type usize.
        let length: usize = (bytecodes[*byte_idx + 1] + bytecodes[*byte_idx + 2]) as usize;
        let utf8_start_byte = *byte_idx + 3;
        let utf8_end_byte = *byte_idx + 3 + length;
        let utf8_byte_slice: &[u8] = &bytecodes[utf8_start_byte..utf8_end_byte];
        let utf8_str = match str::from_utf8(utf8_byte_slice) {
                Ok(n) => n,
                Err(e) => panic!("[ERROR] Expected utf8 string, but is not valid: {:?}", e),
        };
        let entry = CONSTANT_Utf8 {
            utf8_str: utf8_str,
        };
        *byte_idx = utf8_end_byte;
        entry
    }
}
