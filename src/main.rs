extern crate rustedjvm;

use std::env;

use rustedjvm::classes::*;
use rustedjvm::interpreter;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: $ rusted_jvm <ClassNameToRun>");
        return;
    };

    run(&args[1]);
}

fn run(class_name: &String) {
    let class_file = ClassFile::new(class_name);
    let class = class_file.parse();

    interpreter::run(class);
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn test_simple_addition() {
        run(&String::from("test/SimpleAddition"));
    }

    /*#[test]
    fn test_hello_world() {
        run(&String::from("test/HelloWorld"));
    }*/
}
