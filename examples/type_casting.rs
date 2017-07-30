extern crate llvm;

use llvm::ContextType;

fn main() {
    let context = llvm::Context::new();

    let integer_type = i16::get_type_in_context(&context);
    println!("{:?} {}", integer_type, integer_type.width());
    let generic_type: &llvm::Type = integer_type; // upcast
    println!("{:?}", generic_type);

    if let llvm::types::Kind::Integer(t) = generic_type.downcast() {
        println!("{:?} {}", t, t.width());
    }
}
