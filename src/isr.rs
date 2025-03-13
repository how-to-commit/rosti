use crate::println;

pub extern "x86-interrupt" fn general_fault() {
    println!("fault!");
}
