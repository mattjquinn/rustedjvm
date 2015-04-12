pub struct Method {
    pub access_flags: u16,
}

impl Method {
    pub fn from_bytecodes(bytecodes: &Vec<u8>, byte_idx: &mut usize) -> Method {
        let method = Method {
            access_flags: (bytecodes[*byte_idx] + bytecodes[*byte_idx + 1]) as u16,
        };
        *byte_idx = *byte_idx + 2;
        method
    }

    pub fn to_string(&self) -> String {
        format!("Method[access_flags=0x{:x}]", self.access_flags)
    }
}
