extern crate rustedjvm;

use std::env;

use rustedjvm::classes::Class;
use rustedjvm::interpreter;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: $ rusted_jvm <ClassNameToRun>");
        return;
    }

    let ref class_name = args[1];
    let class: Class = Class::from_class_file(class_name);

    interpreter::run(&class);
}
