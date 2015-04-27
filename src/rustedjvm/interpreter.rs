use attributes::*;
use classes::*;

struct Object<'a> {
    class: &'a Class<'a>,
}

pub fn run(class: Class) {
    if !class.methods.contains_key("main") {
        panic!("[ERROR] Class provided to interpreter does not have \
                a main method.");
    };

    let init_method = match class.methods.get("<init>") {
        Some(e) => e,
        None => panic!("[ERROR] Class lacks <init> method."),
    };

    let code_attr = match init_method.attributes.get("Code") {
        Some(&Attribute::Code(ref s)) => s,
        _ => panic!("[ERROR] Code attribute not found."),
    };

    /*
     * Set up the local variable array;
     * the first entry of the local variable array is always
     * the "this" reference.
     */
    let mut local_var_arr = Vec::new();
    let this_ref = Object {class: &class};
    local_var_arr.push(this_ref);

    for bytecode in code_attr.code_slice {
        println!("Bytecode: {:x}", bytecode);
    }

    println!("TODO: Run <init> bytecodes.");
}
