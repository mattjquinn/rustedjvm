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

    println!("TODO: Interpret init_method.");
}
