use attributes::*;
use classes::*;
use constants::*;

struct Object<'a> {
    class: &'a Class<'a>,
}

enum Operand<'a> {
    Ref(&'a Object<'a>),
    Int(i32),
}

pub fn run(class: Class) {
    if !class.methods.contains_key("main") {
        panic!("[ERROR] Class provided to interpreter does not have \
                a main method.");
    };

    let object = Object { class: &class };
    run_method(&object, "<init>");
    run_method(&object, "main");
}

fn run_method(obj: &Object, method_name: &str) -> () {

    let method = match obj.class.methods.get(method_name) {
        Some(e) => e,
        None => panic!("[ERROR] Class lacks method: {}", method_name),
    };


    let code_attr = match method.attributes.get("Code") {
        Some(&Attribute::Code(ref s)) => s,
        _ => panic!("[ERROR] Code attribute not found."),
    };

    /*
     * Set up the local variable array;
     * the first entry of the local variable array is always
     * the "this" reference to the contextual object.
     */
    let mut local_var_arr : Vec<Operand>
        = Vec::with_capacity(code_attr.max_locals as usize);
    local_var_arr.push(Operand::Ref(&obj));

    /*
     * Set up the operand stack, which is initially empty.
     */
    let mut operand_stack: Vec<&Operand>
        = Vec::with_capacity(code_attr.max_stack as usize);

    /*
     * Begin executing method bytecodes.
     */
    println!("Interpreting...");
    let mut bytecode_idx: usize = 0;
    while bytecode_idx < code_attr.code_slice.len() {
        match code_attr.code_slice[bytecode_idx] {
            0x06 => {
                iconst_3(&mut operand_stack);
            },
            0x2a => {
                aload_0(&local_var_arr, &mut operand_stack);
                bytecode_idx += 1;
            },
            0xb1 => {
                return;
            },
            0xb2 => {
                getstatic(&obj, &mut operand_stack,
                          code_attr.code_slice[bytecode_idx+1] as u16,
                          code_attr.code_slice[bytecode_idx+2] as u16);
                bytecode_idx += 3;
            },
            0xb7 => {
                invokespecial(&mut operand_stack,
                              code_attr.code_slice[bytecode_idx+1] as u16,
                              code_attr.code_slice[bytecode_idx+2] as u16);
                bytecode_idx += 3;
            },
            unsup_code => panic!("[ERROR] Encountered unsupported \
                                  bytecode: {:x}", unsup_code),
        }
    }
}

fn iconst_3(operand_stack: &mut Vec<&Operand>) {
    println!("iconst_3");
    panic!("[TODO] In iconst_3, push integer literal 3 onto stack.");
}

fn aload_0<'a, 'b>(local_var_arr: &'b Vec<Operand<'a>>,
               operand_stack: &mut Vec<&'b Operand<'a>>) {
    println!("aload_0");
    operand_stack.push(&local_var_arr[0]);
}

fn invokespecial(operand_stack: &mut Vec<&Operand>,
                 indexbyte1: u16,
                 indexbyte2: u16) {
    let object_ref: &Object = match operand_stack.pop() {
        Some(&Operand::Ref(ref e)) => e,
        Some(&Operand::Int(_)) => panic!("[ERROR] invokespecial \
                does not yet support integers."),
        None => panic!("[ERROR] Expected objectref, found None."),
    };

    /*
     * Join the two index bytes according to the JLS,
     * then traverse the appropriate entries in the constant pool
     * in order to determine the method name to invoke, along
     * with the associated class name and method signature.
     */
    let method_const_idx: u16 = (indexbyte1 << 8) | indexbyte2;
    let method_const = match object_ref
            .class.constant_pool.get(&method_const_idx) {
        Some(&ConstantPoolEntry::MethodRef(ref e)) => e,
        _ => panic!("[ERROR] Expected method ref in constant \
                     pool at index {}.", method_const_idx),
    };
    let class_const = match object_ref
            .class.constant_pool.get(&method_const.class_idx) {
        Some(&ConstantPoolEntry::Class(ref e)) => e,
        _ => panic!("[ERROR] Expected class in constant pool \
                     at index {}.", method_const.class_idx),
    };
    let class_name = match object_ref
            .class.constant_pool.get(&class_const.name_idx) {
        Some(&ConstantPoolEntry::Utf8(ref e)) => e,
        _ => panic!("[ERROR] Expected utf8 in constant pool \
                     at index {}.", class_const.name_idx),
    };
    let name_type_const = match object_ref
            .class.constant_pool.get(&method_const.name_and_type_idx) {
        Some(&ConstantPoolEntry::NameAndType(ref e)) => e,
        _ => panic!("[ERROR] Expected name/type in constant pool \
                     at index {}.", method_const.name_and_type_idx),
    };
    let method_name = match object_ref
            .class.constant_pool.get(&name_type_const.name_idx) {
        Some(&ConstantPoolEntry::Utf8(ref e)) => e,
        _ => panic!("[ERROR] Expected utf8 in constant pool \
                     at index {}.", name_type_const.name_idx),
    };
    let method_descriptor = match object_ref
            .class.constant_pool.get(&name_type_const.descriptor_idx) {
        Some(&ConstantPoolEntry::Utf8(ref e)) => e,
        _ => panic!("[ERROR] Expected utf8 in constant pool \
                     at index {}.", name_type_const.descriptor_idx),
    };
    println!("invokespecial: Method {}.\"{}\":{}",
             class_name.utf8_str,
             method_name.utf8_str,
             method_descriptor.utf8_str);
    if class_name.utf8_str == "java/lang/Object"
        && method_name.utf8_str == "<init>"
        && method_descriptor.utf8_str == "()V" {
        /*
         * For now, do nothing. Eventually, this
         * conditional will be removed when it makes
         * sense to add support for dynamic class loading.
         */
    } else {
        panic!("[TODO] Encountered a method other than Object.<init>; \
                this means it's time to support dynamic class loading.");
    };
}

fn getstatic(object_ref: &Object, operand_stack: &mut Vec<&Operand>,
             indexbyte1: u16,
             indexbyte2: u16) {
    /*
     * Join the two index bytes according to the JLS,
     * then traverse the appropriate entries in the constant pool
     * in order to determine the static field to retrieve, along
     * with the associated class name and method signature.
     */
    let field_const_idx: u16 = (indexbyte1 << 8) | indexbyte2;
    let field_const = match object_ref
            .class.constant_pool.get(&field_const_idx) {
        Some(&ConstantPoolEntry::FieldRef(ref e)) => e,
        _ => panic!("[ERROR] Expected field ref in constant \
                     pool at index {}.", field_const_idx),
    };
    let class_const = match object_ref
            .class.constant_pool.get(&field_const.class_idx) {
        Some(&ConstantPoolEntry::Class(ref e)) => e,
        _ => panic!("[ERROR] Expected class in constant pool \
                     at index {}.", field_const.class_idx),
    };
    let class_name = match object_ref
            .class.constant_pool.get(&class_const.name_idx) {
        Some(&ConstantPoolEntry::Utf8(ref e)) => e,
        _ => panic!("[ERROR] Expected utf8 in constant pool \
                     at index {}.", class_const.name_idx),
    };
    let name_type_const = match object_ref
            .class.constant_pool.get(&field_const.name_and_type_idx) {
        Some(&ConstantPoolEntry::NameAndType(ref e)) => e,
        _ => panic!("[ERROR] Expected name/type in constant pool \
                     at index {}.", field_const.name_and_type_idx),
    };
    let field_name = match object_ref
            .class.constant_pool.get(&name_type_const.name_idx) {
        Some(&ConstantPoolEntry::Utf8(ref e)) => e,
        _ => panic!("[ERROR] Expected utf8 in constant pool \
                     at index {}.", name_type_const.name_idx),
    };
    let field_descriptor = match object_ref
            .class.constant_pool.get(&name_type_const.descriptor_idx) {
        Some(&ConstantPoolEntry::Utf8(ref e)) => e,
        _ => panic!("[ERROR] Expected utf8 in constant pool \
                     at index {}.", name_type_const.descriptor_idx),
    };
    println!("getstatic: Field {}.{}:{}",
             class_name.utf8_str,
             field_name.utf8_str,
             field_descriptor.utf8_str);

    panic!("TODO: Push static method/field value onto stack.");
}
