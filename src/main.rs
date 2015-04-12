#![feature(slice_patterns)]

extern crate rustedjvm;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
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
        let const_pool_entry = match ConstantPoolEntry::from_bytecodes(&bytecodes, &mut byte_idx) {
            Ok(entry) => entry,
            Err(error) => panic!("[ERROR] Failed to get constant pool entry: {:?}", error),
        };

        println!("{}{}:\t{}", indent, n, const_pool_entry.to_string());
    };
}
