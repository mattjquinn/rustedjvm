extern crate rustedjvm;

use std::env;

use rustedjvm::classes::*;
use rustedjvm::interpreter;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: $ rusted_jvm <ClassNameToRun>");
        return;
    }

    let ref class_name = args[1];
    let class_file = ClassFile::new(class_name);
    let class = class_file.parse();

    interpreter::run(class);
}
