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
    let mut local_var_arr
        = Vec::with_capacity(code_attr.max_locals as usize);
    let this_ref = Object {class: &class};
    local_var_arr.push(this_ref);

    /*
     * Set up the operand stack, which is initially empty.
     */
    let mut operand_stack: Vec<&Object>
        = Vec::with_capacity(code_attr.max_stack as usize);

    /*
     * Begin executing method bytecodes.
     */
    println!("Interpreting...");
    let mut bytecode_idx: usize = 0;
    while bytecode_idx < code_attr.code_slice.len() {
        match code_attr.code_slice[bytecode_idx] {
            0x2a => {
                aload_0(&local_var_arr, &mut operand_stack);
                bytecode_idx += 1;
            },
            0xb7 => {
                invokespecial(&mut operand_stack,
                              code_attr.code_slice[bytecode_idx+1],
                              code_attr.code_slice[bytecode_idx+2]);
                bytecode_idx += 3;
            },
            unsup_code => panic!("[ERROR] Encountered unsupported \
                                  bytecode: {:x}", unsup_code),
        }
    }
}

fn aload_0<'a, 'b>(local_var_arr: &'b Vec<Object<'a>>,
               operand_stack: &mut Vec<&'b Object<'a>>) {
    println!("aload_0");
    operand_stack.push(&local_var_arr[0]);
}

fn invokespecial(operand_stack: &mut Vec<&Object>,
                 indexbyte1: u8,
                 indexbyte2: u8) {
    println!("invokespecial");
    let object_ref: &Object = match operand_stack.pop() {
        Some(e) => e,
        None => panic!("[ERROR] Expected objectref, found None."),
    };
    println!("indexbyte1: {}", indexbyte1);
    println!("indexbyte2: {}", indexbyte2);
}
