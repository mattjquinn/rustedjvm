use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("HelloWorld.class");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display,
                Error::description(&why)),
        Ok(file) => file,
    };

    let mut v = Vec::new();
    match file.read_to_end(&mut v) {
        Err(why) => panic!("couldn't read {}: {}", display,
                Error::description(&why)),
        Ok(_) => print!("{} contains:\n{}\n", display, v.len()),
    }
}
