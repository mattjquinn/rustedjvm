#![feature(slice_patterns)]

extern crate rustedjvm;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::str;
use rustedjvm::constant_pool::*;

fn main() {
    let path = Path::new("HelloWorld.class");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("[ERROR] Unable to open {}: {}",
                display, Error::description(&why)),
        Ok(file) => file,
    };

    let mut bytecodes = Vec::new();
    match file.read_to_end(&mut bytecodes) {
        Err(why) => panic!("[ERROR] Unable to read {}: {}",
                display, Error::description(&why)),
        Ok(_) => println!("{} contains {} bytes.", display, bytecodes.len()),
    };

    let magic_slice = &bytecodes[0..4];
    match magic_slice {
        [0xca, 0xfe, 0xba, 0xbe] => println!("Magic header is present."),
        _ => panic!("[ERROR] File is not a valid class file (magic header absent)."),
    };

    let minor_version = bytecodes[4] + bytecodes[5];
    let major_version = bytecodes[6] + bytecodes[7];

    println!("Major version: {}, minor version: {}", major_version, minor_version);

    let constant_pool_size = bytecodes[8] + bytecodes[9];

    println!("Constant Pool (Count: {})", constant_pool_size);
    println!("===================================================");

    let mut byte_idx = 10;
    let indent = "  ";
    for n in 1 .. constant_pool_size {
        match bytecodes[byte_idx] {
            0x1 => {
                // Bytecodes are u8, but slicing requires arguments of type usize.
                let length: usize = (bytecodes[byte_idx + 1] + bytecodes[byte_idx + 2]) as usize;
                let utf8_start_byte = byte_idx + 3;
                let utf8_end_byte = byte_idx + 3 + length;
                let utf8_str = match str::from_utf8(&bytecodes[utf8_start_byte..utf8_end_byte]) {
                        Ok(n) => n,
                        Err(e) => panic!("[ERROR] Expected utf8 string, but is not valid: {:?}", e),
                };
                println!("{}{}:\tCONSTANT_Utf8[length={}, utf8_str=\"{}\"]",
                         indent, n, length, utf8_str);
                byte_idx = utf8_end_byte;
            },
            0x7 => {
                let entry = CONSTANT_Class::from_bytecodes(&bytecodes, &mut byte_idx);
                println!("{}{}:\tCONSTANT_Class[name_index={}]",
                         indent, n, entry.name_idx);
            },
            0x8 => {
                let entry = CONSTANT_String::from_bytecodes(&bytecodes, &mut byte_idx);
                println!("{}{}:\tCONSTANT_String[string_index={}]",
                         indent, n, entry.string_idx);
            },
            0x9 => {
                let entry = CONSTANT_FieldRef::from_bytecodes(&bytecodes, &mut byte_idx);
                println!("{}{}:\tCONSTANT_FieldRef[class_index={}, name_and_type={}]",
                        indent, n, entry.class_idx, entry.name_and_type_idx);
            },
            0xa => {
                let class_index = bytecodes[byte_idx + 1] + bytecodes[byte_idx + 2];
                let name_and_type = bytecodes[byte_idx + 3] + bytecodes[byte_idx + 4];
                println!("{}{}:\tCONSTANT_MethodRef[class_index={}, name_and_type={}]",
                        indent, n, class_index, name_and_type);
                byte_idx = byte_idx + 5;
            },
            unsupported_code => panic!("[ERROR] Constant pool entry \
                    not supported: 0x{:x}", unsupported_code),
        };
    };
}
