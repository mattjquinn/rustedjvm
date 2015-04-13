pub struct Method {
    pub access_flags: u16,
    pub name_idx: u16,
    pub descriptor_idx: u16,
    pub attrs_count: u16,
}

impl Method {
    pub fn from_bytecodes(bytecodes: &Vec<u8>, byte_idx: &mut usize) -> Method {
        let method = Method {
            access_flags: (bytecodes[*byte_idx] + bytecodes[*byte_idx + 1]) as u16,
            name_idx: (bytecodes[*byte_idx + 2] + bytecodes[*byte_idx + 3]) as u16,
            descriptor_idx: (bytecodes[*byte_idx + 4] + bytecodes[*byte_idx + 5]) as u16,
            attrs_count: (bytecodes[*byte_idx + 6] + bytecodes[*byte_idx + 7]) as u16,
        };
        *byte_idx = *byte_idx + 8;
        method
    }

    pub fn to_string(&self) -> String {
        format!("Method\n\
                \t\t- access_flags=0x{:x}\n\
                \t\t- name_idx={}\n\
                \t\t- descriptor_idx={}\n\
                \t\t- attrs_count={}",
                self.access_flags, self.name_idx, self.descriptor_idx, self.attrs_count)
    }
}
