use attributes::*;
use classes::*;

pub fn run(class: Class) {
    if !class.methods.contains_key("main") {
        panic!("[ERROR] Class provided to interpreter does not have \
                a main method.");
    };

    let init_method = match class.methods.get("<init>") {
        Some(e) => e,
        None => panic!("[ERROR] Class lacks <init> method."),
    };

    let init_code_length = match init_method.attributes.get("Code") {
        Some(&Attribute::Code(ref s)) => s.code_length,
        _ => panic!("[ERROR] Code length not found."),
    };

    println!("<init> code length: {}", init_code_length);
    println!("TODO: Run <init> bytecodes.");
}
