use std::str;

pub enum ConstantPoolEntry<'a> {
    Utf8(Utf8Constant<'a>),
    Class(ClassConstant),
    String(StringConstant),
    FieldRef(FieldRefConstant),
    MethodRef(MethodRefConstant),
    NameAndType(NameAndTypeConstant),
}

pub struct ClassConstant {
    pub name_idx: u16,
}

pub struct StringConstant {
    pub string_idx: u16,
}

pub struct FieldRefConstant {
    pub class_idx: u16,
    pub name_and_type_idx: u16,
}

pub struct MethodRefConstant {
    pub class_idx: u16,
    pub name_and_type_idx: u16,
}

pub struct NameAndTypeConstant {
    pub name_idx: u16,
    pub descriptor_idx: u16,
}

// Lifetime must be made explict
// here because utf8_str is only valid
// for as long as the underlying bytecode array lives.
pub struct Utf8Constant<'a> {
    pub utf8_str: &'a str,
}

impl<'a> ConstantPoolEntry<'a> {
    pub fn from_bytecodes(bytecodes: &'a Vec<u8>, byte_idx: &mut usize)
            -> Result<ConstantPoolEntry<'a>, String> {
        match bytecodes[*byte_idx] {
            0x1 => Ok(ConstantPoolEntry::Utf8(
                    Utf8Constant::from_bytecodes(bytecodes, byte_idx))),
            0x7 => Ok(ConstantPoolEntry::Class(
                    ClassConstant::from_bytecodes(bytecodes, byte_idx))),
            0x8 => Ok(ConstantPoolEntry::String(
                    StringConstant::from_bytecodes(bytecodes, byte_idx))),
            0x9 => Ok(ConstantPoolEntry::FieldRef(
                    FieldRefConstant::from_bytecodes(bytecodes, byte_idx))),
            0xa => Ok(ConstantPoolEntry::MethodRef(
                    MethodRefConstant::from_bytecodes(bytecodes, byte_idx))),
            0xc => Ok(ConstantPoolEntry::NameAndType(
                    NameAndTypeConstant::from_bytecodes(bytecodes, byte_idx))),
            unsupported_code => Err(format!(
                    "Unsupported bytecode 0x{:x}", unsupported_code)),
        }
    }

    pub fn to_string(&self) -> String {
        match *self {
            /*
             * For all of the below matches, a reference to
             * the underlying struct must be used; this is because we have
             * borrowed self, and thus cannot take
             * ownership of anything owned by self.
             */
            ConstantPoolEntry::Utf8(ref s) => format!(
                "Utf8Constant[utf8_str=\"{}\"]", s.utf8_str),
            ConstantPoolEntry::Class(ref s) => format!(
                "ClassConstant[name_index={}]", s.name_idx),
            ConstantPoolEntry::String(ref s) => format!(
                "StringConstant[string_index={}]", s.string_idx),
            ConstantPoolEntry::FieldRef(ref s) => format!(
                "FieldRefConstant[class_idx={}, name_and_type_idx={}]",
                    s.class_idx, s.name_and_type_idx),
            ConstantPoolEntry::MethodRef(ref s) => format!(
                "MethodRefConstant[class_idx={}, name_and_type_idx={}]",
                    s.class_idx, s.name_and_type_idx),
            ConstantPoolEntry::NameAndType(ref s) => format!(
                "NameAndTypeConstant[name_idx={}, descriptor_idx={}]",
                    s.name_idx, s.descriptor_idx),
        }
    }
}

impl<'a> Utf8Constant<'a> {

    // The explict 'a lifetime tags link the bytecode
    // array with the returned struct,
    // because the string slice reference is only
    // valid as long as the bytecode array is alive.
    pub fn from_bytecodes(bytecodes: &'a Vec<u8>,
                          byte_idx: &mut usize) -> Utf8Constant<'a> {
        // Bytecodes are u8, but slicing requires arguments of type usize.
        let length: usize = (bytecodes[*byte_idx + 1]
                             + bytecodes[*byte_idx + 2]) as usize;
        let utf8_start_byte = *byte_idx + 3;
        let utf8_end_byte = *byte_idx + 3 + length;
        let utf8_byte_slice: &[u8] = &bytecodes[utf8_start_byte..utf8_end_byte];
        let utf8_str = match str::from_utf8(utf8_byte_slice) {
                Ok(n) => n,
                Err(e) => panic!("[ERROR] Expected utf8 string, \
                                 but is not valid: {:?}", e),
        };
        let entry = Utf8Constant {
            utf8_str: utf8_str,
        };
        *byte_idx = utf8_end_byte;
        entry
    }
}

impl ClassConstant {
    pub fn from_bytecodes(bytecodes: &Vec<u8>,
                          byte_idx: &mut usize) -> ClassConstant {
        let entry = ClassConstant {
            name_idx: (bytecodes[*byte_idx + 1]
                       + bytecodes[*byte_idx + 2]) as u16,
        };
        *byte_idx = *byte_idx + 3;
        entry
    }
}

impl StringConstant {
    pub fn from_bytecodes(bytecodes: &Vec<u8>,
                          byte_idx: &mut usize) -> StringConstant {
        let entry = StringConstant {
            string_idx: (bytecodes[*byte_idx + 1]
                         + bytecodes[*byte_idx + 2]) as u16,
        };
        *byte_idx = *byte_idx + 3;
        entry
    }
}

impl FieldRefConstant {
    pub fn from_bytecodes(bytecodes: &Vec<u8>,
                          byte_idx: &mut usize) -> FieldRefConstant {
        let entry = FieldRefConstant {
            class_idx: (bytecodes[*byte_idx + 1]
                        + bytecodes[*byte_idx + 2]) as u16,
            name_and_type_idx: (bytecodes[*byte_idx + 3]
                                + bytecodes[*byte_idx + 4]) as u16,
        };
        *byte_idx = *byte_idx + 5;
        entry
    }
}

impl MethodRefConstant {
    pub fn from_bytecodes(bytecodes: &Vec<u8>,
                          byte_idx: &mut usize) -> MethodRefConstant {
        let entry = MethodRefConstant {
            class_idx: (bytecodes[*byte_idx + 1]
                        + bytecodes[*byte_idx + 2]) as u16,
            name_and_type_idx: (bytecodes[*byte_idx + 3]
                                + bytecodes[*byte_idx + 4]) as u16,
        };
        *byte_idx = *byte_idx + 5;
        entry
    }
}

impl NameAndTypeConstant {
    pub fn from_bytecodes(bytecodes: &Vec<u8>,
                          byte_idx: &mut usize) -> NameAndTypeConstant {
        let entry = NameAndTypeConstant {
            name_idx: (bytecodes[*byte_idx + 1]
                       + bytecodes[*byte_idx + 2]) as u16,
            descriptor_idx: (bytecodes[*byte_idx + 3]
                             + bytecodes[*byte_idx + 4]) as u16,
        };
        *byte_idx = *byte_idx + 5;
        entry
    }
}
