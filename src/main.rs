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

    let mut v = Vec::new();
    match file.read_to_end(&mut v) {
        Err(why) => panic!("[ERROR] Unable to read {}: {}",
                display, Error::description(&why)),
        Ok(_) => println!("{} contains {} bytes.", display, v.len()),
    };

    let magic_slice = &v[0..4];
    match magic_slice {
        [0xca, 0xfe, 0xba, 0xbe] => println!("Magic header is present."),
        _ => panic!("[ERROR] File is not a valid class file (magic header absent)."),
    }

    for x in v.iter() {
        println!("{:x}", x);
    }
}
