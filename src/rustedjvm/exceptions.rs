pub struct ExceptionTableEntry {
    start_pc: u16,
}

impl ExceptionTableEntry {
    pub fn from_bytecodes(bytecodes: &Vec<u8>, byte_idx: &mut usize)
                            -> ExceptionTableEntry {

        let start_pc = (bytecodes[*byte_idx]
                             + bytecodes[*byte_idx + 1]) as u16;
        *byte_idx = *byte_idx + 2;

        ExceptionTableEntry {
            start_pc: start_pc
        }
    }

    pub fn to_string(&self) -> String {
        format!("ExceptionTableEntry:\n\
                \t\t\t- start_pc={}",
                self.start_pc)
    }
}
