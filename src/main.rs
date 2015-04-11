#![feature(slice_patterns)]

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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

    println!("Constant pool count: {}", constant_pool_size);

    //for x in v.iter() {
    //    println!("{:x}", x);
    //}
}
