#![feature(slice_patterns)]

extern crate rustedjvm;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;
use rustedjvm::constant_pool::*;
use rustedjvm::methods::*;

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

    // The JVM spec states that the number of entries in the constant pool is actually
    // one less than the actual count, hence the subtraction by 1:
    println!("BEGIN Constant Pool (Count: {})", constant_pool_size - 1);
    println!("===================================================");

    let mut byte_idx = 10;
    let indent = "  ";
    let mut constant_pool = HashMap::new();

    // The JVM spec states that the number of entries in the constant
    // pool is actually one less than the actual count, and that entries
    // start at index 1. Hence: "1 .. constant_pool_size (latter bound is exclusive)"
    for n in 1 .. constant_pool_size {
        let const_pool_entry = match ConstantPoolEntry::from_bytecodes(&bytecodes, &mut byte_idx) {
            Ok(entry) => entry,
            Err(error) => panic!("[ERROR] Failed to get constant pool entry: {:?}", error),
        };

        println!("{}{}:\t{}", indent, n, const_pool_entry.to_string());
        constant_pool.insert(n as u16, const_pool_entry);
    };

    println!("END Constant Pool");
    println!("===================================================");

    let access_flags = bytecodes[byte_idx] + bytecodes[byte_idx + 1];
    byte_idx = byte_idx + 2;
    println!("Access flags: 0x{:x}", access_flags);

    let this_class_const_pool_entry_idx = bytecodes[byte_idx] + bytecodes[byte_idx + 1];
    byte_idx = byte_idx + 2;
    println!("This class' constant pool entry idx: 0x{:x}", this_class_const_pool_entry_idx);

    let super_class_const_pool_entry_idx = bytecodes[byte_idx] + bytecodes[byte_idx + 1];
    byte_idx = byte_idx + 2;
    println!("Super class' constant pool entry idx: 0x{:x}", super_class_const_pool_entry_idx);

    let interface_count = bytecodes[byte_idx] + bytecodes[byte_idx + 1];
    byte_idx = byte_idx + 2;
    println!("Interface count: {}", interface_count);

    assert!(interface_count == 0, "[ERROR] Classes w/ interfaces are not yet supported.");

    let field_count = bytecodes[byte_idx] + bytecodes[byte_idx + 1];
    byte_idx = byte_idx + 2;
    println!("Field count: {}", field_count);

    assert!(field_count == 0, "[ERROR] Classes w/ fields are not yet supported.");

    let method_count = bytecodes[byte_idx] + bytecodes[byte_idx + 1];
    byte_idx = byte_idx + 2;
    println!("Method count: {}", method_count);

    println!("BEGIN Methods (Count: {})", method_count);
    println!("===================================================");

    for n in 0 .. method_count {
        let method = Method::from_bytecodes(&bytecodes, &mut byte_idx, &constant_pool);
        println!("{}{}:\t{}", indent, n, method.to_string());
        println!("Byte idx is 0x{:x}", byte_idx);
    };

    println!("END Methods");
    println!("===================================================");

    println!("Byte idx is 0x{:x}", byte_idx);
}
