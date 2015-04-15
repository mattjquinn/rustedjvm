pub struct ExceptionTableEntry {
    start_pc: u16,
    end_pc: u16,
    handle_pc: u16,
    catch_type: u16,
}

impl ExceptionTableEntry {
    pub fn from_bytecodes(bytecodes: &Vec<u8>, byte_idx: &mut usize)
                            -> ExceptionTableEntry {

        let start_pc = (bytecodes[*byte_idx]
                        + bytecodes[*byte_idx + 1]) as u16;
        *byte_idx = *byte_idx + 2;

        let end_pc = (bytecodes[*byte_idx]
                      + bytecodes[*byte_idx + 1]) as u16;
        *byte_idx = *byte_idx + 2;

        let handle_pc = (bytecodes[*byte_idx]
                         + bytecodes[*byte_idx + 1]) as u16;
        *byte_idx = *byte_idx + 2;

        let catch_type = (bytecodes[*byte_idx]
                          + bytecodes[*byte_idx + 1]) as u16;
        *byte_idx + *byte_idx + 2;

        ExceptionTableEntry {
            start_pc: start_pc,
            end_pc: end_pc,
            handle_pc: handle_pc,
            catch_type: catch_type,
        }
    }

    pub fn to_string(&self) -> String {
        format!("ExceptionTableEntry:\n\
                \t\t\t- start_pc={}\n\
                \t\t\t- end_pc={}\n\
                \t\t\t- handle_pc={}\n\
                \t\t\t- catch_type={}",
                self.start_pc, self.end_pc, self.handle_pc, self.catch_type)
    }
}
